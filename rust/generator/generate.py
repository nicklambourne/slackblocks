#!/usr/bin/env python3
"""Generate native Rust values from the shared model; never runs during Cargo builds."""
import argparse
import json
import os
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RUST = ROOT / 'rust'
KINDS = {'string','boolean','int','long','double','number','enum','map','style','object','list','rows','text','textList','stringList'}
NAMES = {'Option':'SelectOption','OptionGroup':'SelectOptionGroup','Confirmation':'ConfirmationDialogue'}
# The foundation intentionally exposes only this reviewed prototype and its
# concrete dependencies. Full conformance is a later, separately reviewed stage.
FOUNDATION = {'PlainText','MarkdownText','Option','ButtonElement','SectionBlock','MessagePayload','TableBlock','RichTextBlock','RichTextSection','RichTextText','RichTextList','TaskCardBlock','PlanBlock','ContainerBlock','HeaderBlock','DividerBlock','DataTableBlock','RawNumber','RawText'}
BOXED = {'Block','Element','InputElement','ContextActionsElement'}
SINGULAR = {'series':'series_entry','data':'point','categories':'category','trigger_actions_on':'trigger_action','child_blocks':'child_block','visible_to_user_ids':'visible_to_user_id'}
KEYWORDS = {'type','match','ref','self','Self','super','crate','mod','use','pub','fn','struct','enum','impl','trait','where','move','async','await','dyn','unsafe','loop','in','as','const','static','return','yield','gen','try','abstract','final','override','priv','typeof','unsized','virtual','box','do','macro','become'}
RESERVED = {'builder','build','into_builder','extensions','extension','new','default','serialize_value','from_wire','try_from'}

def name(s): return NAMES.get(s,s)
def ident(s):
    if not re.fullmatch('[a-z][a-z0-9_]*',s) or s in RESERVED: raise ValueError(f'Invalid/colliding Rust field: {s}')
    return 'r#'+s if s in KEYWORDS else s

def variant(t, role):
    if role == 'Text': return {'PlainText':'Plain','MarkdownText':'Markdown'}[t]
    s=name(t)
    for prefix in ('RichText',) if role.startswith('RichText') else ():
        if s.startswith(prefix): s=s[len(prefix):]
    if role in {'Block','Element','InputElement','ContextActionsElement'}:
        for suffix in ('Block','Element'):
            if s.endswith(suffix): s=s[:-len(suffix)]
    if role == 'Chart' and s.endswith('Chart'): s=s[:-5]
    if role in {'TableCell','DataTableCell'} and s=='RichTextBlock': s='RichText'
    return s

def pascal(s): return ''.join(w.capitalize() for w in s.split('_'))
def literal(v):
    if isinstance(v,bool): return str(v).lower()
    if isinstance(v,str): return json.dumps(v,ensure_ascii=False)+'.into()'
    return str(v)
def rust_type(f):
    k=f['kind']; t=name(f.get('type',''))
    if k not in KINDS: raise ValueError(f'Unknown kind: {k}')
    return {'string':'String','boolean':'bool','int':'i64','long':'i64','double':'f64','number':'JsonNumber','map':'Map<String, Value>','style':'RichTextStyle','enum':t,'object':t,'text':t,'list':f'Vec<{t}>','rows':f'Vec<Vec<{t}>>','textList':f'Vec<{t}>','stringList':'Vec<String>'}[k]
def input_type(f):
    if f['kind']=='text': return name(f['type'])+'Input'
    if f['kind']=='textList': return 'Vec<'+name(f['type'])+'Input>'
    return rust_type(f)
def docs(text):
    return '\n'.join('/// '+line for line in str(text).replace("Slack's official Java SDK",'a JSON transport').splitlines())
def leaves(d,p=''):
    out={}
    for k,v in d.items():
        key=f'{p}.{k}' if p else k
        if isinstance(v,dict):out.update(leaves(v,key))
        elif isinstance(v,int) and not isinstance(v,bool):out[key]=v
        else:raise ValueError(f'Invalid scalar limit: {key}')
    return out

def resolve(model, full=False):
    by_name={t['name']:t for t in model['types']}
    if len(by_name)!=len(model['types']):raise ValueError('Duplicate type')
    all_names=[name(t['name']) for cat in ('types','interfaces','enums') for t in model[cat]]
    if len(set(all_names))!=len(all_names):raise ValueError('Rust type name collision')
    known={t['name'] for cat in ('types','interfaces','enums') for t in model[cat]}
    for t in model['types']:
        names=set(); wires=set()
        for f in t['fields']:
            n=ident(f['wire']);rust_type(f)
            if n in names or f['wire'] in wires:raise ValueError(f'Duplicate field in {t["name"]}')
            names.add(n);wires.add(f['wire'])
            if f.get('type') and f['type'] not in known:raise ValueError(f'Unresolved type: {f["type"]}')
        if not set(t.get('defaults',{})) <= wires:raise ValueError('Unknown default field')
        for f in t['fields']:
            if f['kind']=='enum' and f['wire'] in t['defaults']:
                enum=next(e for e in model['enums'] if e['name']==f['type'])
                if t['defaults'][f['wire']] not in [c['wire'] for c in enum['constants']]:raise ValueError('Invalid enum default')
    selected=set(by_name) if full else set(FOUNDATION)
    while True:
        more={f['type'] for t in selected for f in by_name[t]['fields'] if f.get('type') in by_name}
        if more<=selected:break
        selected|=more
    roles={r['name']:[] for r in model['interfaces']}
    for t in model['types']:
        if t['name'] not in selected:continue
        memberships=set(t['implements'])
        if t['package']=='block':memberships.add('Block')
        if t['package']=='element':memberships.add('Element')
        if not memberships<=roles.keys():raise ValueError('Unknown role')
        for role in memberships:roles[role].append(t)
    for role,types in roles.items():
        tags=[t['wireType'] for t in types]
        variants=[variant(t['name'],role) for t in types]
        if len(tags)!=len(set(tags)) or len(variants)!=len(set(variants)):raise ValueError(f'Ambiguous role: {role}')
    return [t for t in model['types'] if t['name'] in selected],{r:ts for r,ts in roles.items() if ts}

def serde_ingress(n):
    return f'''impl TryFrom<Value> for {n} {{
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {{ Self::from_wire(value, "{n}") }}
}}
impl<'de> Deserialize<'de> for {n} {{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }}
}}
'''

def generate(model, full=False):
    types,roles=resolve(model,full)
    s=['// Generated from spec/model.json; edit the generator, not this file.','use serde::{Serialize, Serializer, Deserialize, Deserializer};','use serde::ser::SerializeMap;','use serde_json::{Map, Value};','use crate::{ErrorCategory, JsonNumber, PlainTextInput, TextInput, RichTextStyle, ValidationError};','use crate::wire::{self, FromWire};']
    exports=[]
    for e in model['enums']:
        n=e['name'];exports.append(n)
        s += [docs(e['description']),'#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]','#[non_exhaustive]',f'pub enum {n} {{']
        for c in e['constants']:s += [docs(c['description']),f'{pascal(c["wire"])},']
        s += ['}',f'impl {n} {{','/// Returns the Slack wire spelling.','pub const fn as_str(self) -> &\'static str { match self {']
        s += [f'Self::{pascal(c["wire"])} => "{c["wire"]}",' for c in e['constants']]
        s += ['}}}',f'impl FromWire for {n} {{','fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> { match value.as_str() {']
        s += [f'Some("{c["wire"]}") => Ok(Self::{pascal(c["wire"])}),' for c in e['constants']]
        s += [f'_ => Err(wire::mismatch(path, "{n}")),','}}}',f'impl Serialize for {n} {{ fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok,S::Error> {{ s.serialize_str(self.as_str()) }} }}',serde_ingress(n),f'impl std::fmt::Display for {n} {{ fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result {{ f.write_str(self.as_str()) }} }}',f'impl std::str::FromStr for {n} {{ type Err=ValidationError; fn from_str(s: &str) -> Result<Self,Self::Err> {{ Self::try_from(Value::String(s.into())) }} }}']
    for role,members in roles.items():
        exports.append(role)
        desc=next(i['description'] for i in model['interfaces'] if i['name']==role)
        s += [docs(desc),'#[derive(Clone, Debug, PartialEq)]','#[non_exhaustive]',f'pub enum {role} {{']
        for t in members:
            n=name(t['name']);typ=f'Box<{n}>' if role in BOXED else n
            s += [docs(t['description']),f'{variant(t["name"],role)}({typ}),']
        s += ['}',f'impl Serialize for {role} {{ fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok,S::Error> {{ match self {{']
        s += [f'Self::{variant(t["name"],role)}(v) => v.serialize(s),' for t in members]
        s += ['}}}',f'impl FromWire for {role} {{ fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {{ match value.get("type").and_then(Value::as_str) {{']
        s += [f'Some("{t["wireType"]}") => {name(t["name"])}::from_wire(value,path).map(Into::into),' for t in members]
        s += [f'_ => Err(wire::mismatch(&format!("{{path}}.type"), "a {role} discriminator")),','}}}',serde_ingress(role)]
        for t in members:
            n=name(t['name']);v=variant(t['name'],role);value='Box::new(value)' if role in BOXED else 'value'
            s += [f'impl From<{n}> for {role} {{ fn from(value: {n}) -> Self {{ Self::{v}({value}) }} }}']
    for t in types:
        n=name(t['name']);b=n+'Builder';exports += [n,b];fs=t['fields']
        s += [docs(t['description']),f'///\n/// [Slack reference]({t["docUrl"]}).',docs(' '.join(t['rules'])) if t['rules'] else '', '#[derive(Clone, Debug, PartialEq)]',f'pub struct {n} {{']
        for f in fs:s.append(f'{ident(f["wire"])}: '+(rust_type(f) if f.get('required') else f'Option<{rust_type(f)}>')+',')
        s += ['extensions: Map<String, Value>,','}',docs(f'Consuming builder for [`{n}`]. Validation runs in [`Self::build`].'),'#[derive(Clone, Debug)]',f'pub struct {b} {{']
        s += [f'{ident(f["wire"])}: Option<{input_type(f)}>,' for f in fs]
        s += ['extensions: Map<String, Value>,','}',f'impl Default for {b} {{ fn default() -> Self {{ Self {{']
        for f in fs:
            v=t['defaults'].get(f['wire']);expr='None'
            if v is not None:
                expr=f'Some({name(f["type"])}::{pascal(v)})' if f['kind']=='enum' else f'Some({literal(v)})'
            s.append(f'{ident(f["wire"])}: {expr},')
        s += ['extensions: Map::new(),','}}}',f'impl {n} {{']
        builder_args='channel: impl Into<String>' if n=='MessagePayload' else ''
        builder_expr=f'{b}::default().channel(channel)' if builder_args else f'{b}::default()'
        s += ['/// Starts a builder with the documented model defaults.',f'pub fn builder({builder_args}) -> {b} {{ {builder_expr} }}']
        if n in {'PlainText','MarkdownText','RawText','RichTextText'}:
            s += ['/// Constructs validated text.','///','/// # Errors','/// Returns a validation error when text violates its field limits.',f'pub fn new(text: impl Into<String>) -> Result<Self, ValidationError> {{ Self::builder().text(text).build() }}']
        s += ['/// Moves every field into an editable builder, preserving explicit omissions.',f'pub fn into_builder(self) -> {b} {{ {b} {{']
        for f in fs:
            id=ident(f['wire']);v=f'Some(self.{id})' if f.get('required') else f'self.{id}'
            if f['kind']=='text':v = f'Some(self.{id}.into())' if f.get('required') else v+'.map(Into::into)'
            if f['kind']=='textList':v = f'Some(self.{id}.into_iter().map(Into::into).collect())' if f.get('required') else v+'.map(|v| v.into_iter().map(Into::into).collect())'
            s.append(f'{id},' if id == v else f'{id}: {v},')
        s += ['extensions: self.extensions,','}}']
        for f in fs:
            id=ident(f['wire']);typ=rust_type(f);k=f['kind'];opt=not f.get('required')
            if k in {'boolean','int','long','double','enum'}:getter=typ;expr=f'self.{id}'
            elif k=='string':getter='&str';expr=f'self.{id}.as_deref()' if opt else f'&self.{id}'
            elif k in {'list','rows','textList','stringList'}:getter='&['+typ[4:-1]+']';expr=f'self.{id}.as_deref()' if opt else f'&self.{id}'
            else:getter='&'+typ;expr=f'self.{id}.as_ref()' if opt else f'&self.{id}'
            s += [docs('Borrows or copies `'+f['wire']+'`. '+f['description']),f'pub fn {id}(&self) -> '+(f'Option<{getter}>' if opt else getter)+f' {{ {expr} }}']
        s += ['/// Borrows checked, unmodeled JSON extension fields.','pub fn extensions(&self) -> &Map<String, Value> { &self.extensions }', 'fn serialize_value<S: Serializer>(&self, serializer: S, tagged: bool) -> Result<S::Ok, S::Error> {','let mut map = serializer.serialize_map(None)?;']
        if t['wireType']:s.append(f'if tagged {{ map.serialize_entry("type", "{t["wireType"]}")?; }}')
        else:s.append('let _ = tagged;')
        for f in fs:
            id=ident(f['wire']);expr=f'&self.{id}'
            if n=='PlanBlock' and id=='tasks':expr='&crate::rules::PlanTasks(&self.tasks)'
            if f.get('required'):s.append(f'map.serialize_entry("{f["wire"]}", {expr})?;')
            else:s.append(f'if let Some(value) = &self.{id} {{ map.serialize_entry("{f["wire"]}", value)?; }}')
        s += ['for (key,value) in &self.extensions { map.serialize_entry(key,value)?; }','map.end()','}','}']
        s += [f'impl {b} {{']
        for f in fs:
            id=ident(f['wire']);k=f['kind'];typ=rust_type(f);inp=input_type(f)
            s += [docs(f['description'])]
            if k in {'list','textList','stringList'}:
                inner=inp[4:-1]
                s += ['/// Replaces the collection; order is preserved.',f'pub fn {id}<I, T>(mut self, values: I) -> Self where I: IntoIterator<Item=T>, T: Into<{inner}> {{ self.{id}=Some(values.into_iter().map(Into::into).collect()); self }}']
                singular=SINGULAR.get(f['wire'], f['wire'][:-1] if f['wire'].endswith('s') else 'add_'+f['wire'])
                s += ['/// Appends one item to the collection.',f'pub fn {ident(singular)}(mut self, value: impl Into<{inner}>) -> Self {{ self.{id}.get_or_insert_with(Vec::new).push(value.into()); self }}']
            elif k=='rows':
                inner=name(f['type'])
                s += ['/// Replaces all rows, preserving row and cell order.',f'pub fn {id}<I,R,T>(mut self, rows: I) -> Self where I: IntoIterator<Item=R>, R: IntoIterator<Item=T>, T: Into<{inner}> {{ self.{id}=Some(rows.into_iter().map(|r| r.into_iter().map(Into::into).collect()).collect()); self }}','/// Appends a complete row.',f'pub fn row<I,T>(mut self, row:I)->Self where I:IntoIterator<Item=T>, T:Into<{inner}> {{ self.{id}.get_or_insert_with(Vec::new).push(row.into_iter().map(Into::into).collect()); self }}']
            elif k in {'string','text','object','number'}:s.append(f'pub fn {id}(mut self, value: impl Into<{inp}>) -> Self {{ self.{id}=Some(value.into()); self }}')
            else:s.append(f'pub fn {id}(mut self, value: {typ}) -> Self {{ self.{id}=Some(value); self }}')
            if not f.get('required'):s += [docs(f'Omits `{f["wire"]}`, including any model default.'),f'pub fn clear_{f["wire"]}(mut self) -> Self {{ self.{id}=None; self }}']
        s += ['/// Adds or replaces an unmodeled extension; reserved names fail at build time.','pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self { self.extensions.insert(key.into(),value.into()); self }','/// Validates all fields and their receiving context.','///','/// # Errors','/// Returns a [`ValidationError`] for missing fields, invalid values, reserved','/// extension names, or unsupported field combinations and child contexts.',f'pub fn build(self) -> Result<{n}, ValidationError> {{']
        for f in fs:
            id=ident(f['wire']);k=f['kind'];v=f'self.{id}'
            if k=='text':
                args='' if f['type']=='PlainText' else str(f.get('coerce')=='plain_text').lower()
                v+=f'.map(|v| v.resolve({args}).map_err(|e| e.at("{n}.{f["wire"]}"))).transpose()?'
            if k=='textList':
                args='' if f['type']=='PlainText' else str(f.get('coerce')=='plain_text').lower()
                v+=f'.map(|vs| vs.into_iter().enumerate().map(|(i,v)| v.resolve({args}).map_err(|e| e.at(&format!("{n}.{f["wire"]}[{{i}}]")))).collect::<Result<Vec<_>,_>>()).transpose()?'
            if f.get('required'):v=f'wire::required({v}, "{n}.{f["wire"]}")?'
            s.append(f'let {id} = {v};')
            if k=='double':s.append(f'if {id}.is_some_and(|v| !v.is_finite()) {{ return Err(ValidationError::new(ErrorCategory::OutOfRange,"{n}.{f["wire"]}","expected a finite number")); }}')
        field_names=', '.join(json.dumps(f['wire']) for f in fs)
        s += [f'wire::extensions(&self.extensions, &[{field_names}], "{n}")?;',f'let value = {n} {{'+','.join(ident(f['wire']) for f in fs)+',extensions:self.extensions};','let wire = serde_json::to_value(&value).map_err(|e| ValidationError::new(ErrorCategory::TypeMismatch,"'+n+'",e.to_string()))?;']
        if n in {'PlainText','MarkdownText'}:s.append(f'crate::rules::limits(&wire["text"], "text", "{n}.text")?;')
        for f in fs:
            if f.get('limits'):s.append(f'if let Some(v) = wire.get("{f["wire"]}") {{ crate::rules::limits(v,"{f["limits"]}","{n}.{f["wire"]}")?; }}')
            if f['kind']=='style':s.append(f'if let Some(v) = wire.get("{f["wire"]}") {{ crate::rules::style(v, &{json.dumps(f["flags"])}, "{n}.{f["wire"]}")?; }}')
        s += [f'crate::rules::validate("{t["name"]}",&wire,"{n}")?;','Ok(value)','}','}',f'impl Serialize for {n} {{ fn serialize<S: Serializer>(&self, s: S)->Result<S::Ok,S::Error> {{ self.serialize_value(s,true) }} }}',f'impl FromWire for {n} {{ fn from_wire(value: Value, path: &str) -> Result<Self,ValidationError> {{',f'let mut map = wire::object(value,"{t["wireType"]}",path)?;']
        for f in fs:
            id=ident(f['wire']);k=f['kind'];typ=rust_type(f)
            if f['wire'] in t['defaults'] and f.get('required'):s.append(f'if !map.contains_key("{f["wire"]}") {{ map.insert("{f["wire"]}".into(), serde_json::json!({json.dumps(t["defaults"][f["wire"]])})); }}')
            if k=='text':s.append(f'crate::rules::coerce_text(map.get_mut("{f["wire"]}"), "{f.get("coerce", "plain_text")}");')
            if k=='textList':s.append(f'if let Some(Value::Array(vs))=map.get_mut("{f["wire"]}") {{ for v in vs {{ crate::rules::coerce_text(Some(v),"{f.get("coerce", "plain_text")}"); }} }}')
            if n=='PlanBlock' and id=='tasks':s.append('crate::rules::restore_tasks(map.get_mut("tasks"));')
            s.append(f'let {id} = wire::field::<{typ}>(&mut map,"{f["wire"]}",path,false)?;')
        s += [f'{b} {{']
        for f in fs:
            id=ident(f['wire']);v=id
            if f['kind']=='text':v+='.map(Into::into)'
            if f['kind']=='textList':v+='.map(|vs| vs.into_iter().map(Into::into).collect())'
            s.append(f'{id},' if id == v else f'{id}: {v},')
        s += ['extensions:map,','}.build().map_err(|e| e.at(path))','}}',serde_ingress(n)]
    return '\n'.join(s)+'\n',exports,types,roles

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--check',action='store_true');args=parser.parse_args()
    model=json.loads((ROOT/'spec/model.json').read_text())
    full=(RUST/'generator/full-contract').exists()
    source,exports,types,roles=generate(model,full)
    manifest=json.loads((ROOT/'spec/manifest.json').read_text())
    lim=leaves(json.loads((ROOT/'spec/limits.json').read_text()));voc=json.loads((ROOT/'spec/vocabulary.json').read_text())
    constants='// Generated from the shared limits, vocabulary and manifest.\n'
    constants+=f'/// Shared specification implemented by this crate.\npub(crate) const SPEC_VERSION: &str = "{manifest["spec_version"]}";\n'
    constants+='pub(crate) const LIMITS: &[(&str,i64)] = &[\n'+''.join(f'("{k}",{v}),\n' for k,v in sorted(lim.items()))+'];\n'
    constants+='pub(crate) const ICONS: &[&str] = &'+json.dumps(voc['slack_icon_names'])+';\n'
    for surface,blocks in voc['surface_block_types'].items():constants+=f'pub(crate) const {surface.upper()}_BLOCKS: &[&str] = &'+json.dumps(blocks)+';\n'
    outputs={RUST/'src/generated/models.rs':source,RUST/'src/generated/constants.rs':constants,RUST/'src/generated/mod.rs':'// Generated; explicit facade lives in src/lib.rs.\nmod models;\nmod constants;\npub(crate) use constants::*;\npub use models::{'+','.join(exports)+'};\n',RUST/'generated/reference.json':json.dumps({'stage':'complete' if full else 'foundation','types':[dict(t,rustName=name(t['name'])) for t in types],'roles':{r:[name(t['name']) for t in ts] for r,ts in roles.items()},'exports':exports,'naming':NAMES},indent=2)+'\n'}
    rustfmt=os.environ.get('RUSTFMT','rustfmt')
    for p,content in list(outputs.items()):
        if p.suffix=='.rs':
            result=subprocess.run([rustfmt,'--edition','2024','--emit','stdout'],input=content,text=True,capture_output=True,check=True)
            outputs[p]=result.stdout
    owned={RUST/'src/generated',RUST/'generated'}
    unexpected={p for d in owned for p in d.rglob('*') if p.is_file()}-outputs.keys()
    if unexpected:raise SystemExit('Unexpected generated files: '+', '.join(str(p.relative_to(ROOT)) for p in sorted(unexpected)))
    if args.check:
        different=[str(p.relative_to(ROOT)) for p,c in outputs.items() if not p.exists() or p.read_text()!=c]
        if different:raise SystemExit('Generated output differs: '+', '.join(different))
    else:
        for p,c in outputs.items():p.parent.mkdir(parents=True,exist_ok=True);p.write_text(c)

if __name__=='__main__':main()
