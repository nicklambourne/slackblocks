#!/usr/bin/env python3
"""Generate Haskell's native model API directly from the shared contract."""
import argparse
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
PACKAGE = ROOT / "haskell"
NAMES = {"Text": "TextObject", "Option": "SelectOption", "OptionGroup": "SelectOptionGroup", "Confirmation": "ConfirmationDialogue"}


def name(value):
    return NAMES.get(value, value)


def lower(value):
    return value[0].lower() + value[1:]


def camel(value):
    return "".join(part[:1].upper() + part[1:].lower() for part in value.split("_"))


def string(value):
    # Haskell string literals use decimal Unicode escapes rather than JSON's \u.
    return '"' + ''.join('\\"' if c == '"' else '\\\\' if c == '\\' else '\\n' if c == '\n' else c if 32 <= ord(c) < 127 else '\\' + str(ord(c)) + '\\&' for c in value) + '"'


def hslist(values):
    return "[" + ", ".join(values) + "]"


def native(field):
    kind = field["kind"]
    if kind == "string": return "Text"
    if kind == "boolean": return "Bool"
    if kind in {"int", "long"}: return "Integer"
    if kind in {"number", "double"}: return "Scientific"
    if kind in {"object", "text", "enum"}: return name(field["type"])
    if kind == "list" and field["type"] == "TaskCardBlock": return "[PlanTask]"
    if kind in {"list", "textList"}: return "[" + name(field["type"]) + "]"
    if kind == "rows": return "[[" + name(field["type"]) + "]]"
    if kind == "stringList": return "[Text]"
    if kind == "map": return "Map Text Value"
    if kind == "style": return "RichTextStyle"
    raise ValueError(f"Unknown field kind {kind}")


def fieldname(t, f):
    return lower(name(t["name"])) + f["method"][:1].upper() + f["method"][1:]


def default(t, f, enums):
    value = t.get("defaults", {}).get(f["wire"])
    if value is None: return "Nothing"
    if f["kind"] == "enum": value = name(f["type"]) + camel(value)
    elif isinstance(value, bool): value = str(value)
    elif isinstance(value, str): value = string(value)
    else: value = str(value)
    return "Just (" + value + ")"


def resolve(model):
    names = [name(x['name']) for key in ('types', 'enums', 'interfaces') for x in model[key]]
    if len(names) != len(set(names)): raise ValueError("Type name collision")
    for enum in model['enums']:
        constructors = [camel(c['wire']) for c in enum['constants']]
        if not constructors or len(constructors) != len(set(constructors)):
            raise ValueError("Enum constructor collision or empty enum")
    roles = {r['name']: [] for r in model['interfaces']}
    for t in model['types']:
        fields = [fieldname(t, f) for f in t['fields']]
        if len(fields) != len(set(fields)): raise ValueError("Field name collision")
        for f in t['fields']:
            native(f)
            if f.get('type') and name(f['type']) not in names: raise ValueError("Unknown field type")
        memberships = set(t['implements'])
        if t['package'] == 'block': memberships.add('Block')
        if t['package'] == 'element': memberships.add('Element')
        for r in sorted(memberships): roles[r].append(t)
    for role, members in roles.items():
        tags = [t['wireType'] for t in members]
        if len(set(tags)) != len(tags): raise ValueError(f"Ambiguous role {role}")
    return {r: ts for r, ts in roles.items() if ts}


def generate(model):
    roles = resolve(model)
    exports = ['packageVersion', 'specVersion']
    lines = []
    enums = {e['name']: e for e in model['enums']}
    for e in model['enums']:
        n = name(e['name'])
        exports.append(n + '(..)')
        cs = [(n + camel(c['wire']), c['wire']) for c in e['constants']]
        lines += ['-- | ' + e['description'], 'data ' + n + '\n  = ' + '\n  | '.join(c + ' -- ^ Slack wire value: @' + w + '@.' for c, w in cs) + '\n  deriving (Eq, Ord, Show, Enum, Bounded)', f'instance ToJSON {n} where']
        lines += [f'  toJSON {c} = String {string(w)}' for c, w in cs]
        lines += [f'instance FromSlackJSON {n} where', '  fromSlackJSON value = case value of']
        lines += [f'    String {string(w)} -> Right {c}' for c, w in cs]
        lines += ['    _ -> failure TypeMismatch [] "enum" "unknown enumeration value"', f'instance FromJSON {n} where parseJSON = checkedParser', '']
    for t in model['types']:
        n = name(t['name']); factory = lower(n); fs = t['fields']; required = [f for f in fs if f.get('required')]
        exports += [n, n+'Fields(..)', factory, 'build'+n, 'inspect'+n]
        lines += ['-- | ' + t['description'], '--', '-- <' + t['docUrl'] + '>']
        lines += ['-- ' + r for r in t['rules']]
        lines += [f'newtype {n} = Checked{n} {n}Fields deriving (Eq, Show)', f'-- | Editable construction input. Call \'build{n}\' before encoding.', f'data {n}Fields = {n}Fields']
        fielddecls = [(fieldname(t,f), native(f) if f.get('required') else 'Maybe ('+native(f)+')',f['description'] + (' Required.' if f.get('required') else ' Optional; Nothing omits the field. Factory default: @' + default(t, f, enums) + '@.') + (' Shared limit: @' + f['limits'] + '@.' if f.get('limits') else '')) for f in fs]
        fielddecls.append((factory+'Extensions', 'Map Text Value', 'Unmodeled extension fields. Reserved and modeled keys are rejected.'))
        for i,(fn,typ,desc) in enumerate(fielddecls):
            lines += [('  { ' if i==0 else '  , ') + fn + ' :: ' + typ + ' -- ^ ' + desc]
        lines += ['  } deriving (Eq, Show)', f'-- | Start editable fields for {n}; required arguments follow the signature.', '-- Optional fields start absent except the explicit shared defaults; no block ID is invented.', f'-- Finish with \'build{n}\' to validate before JSON encoding.', f'{factory} :: ' + ' -> '.join([native(f) for f in required]+[n+'Fields'])]
        args = ['arg'+str(i) for i in range(len(required))]
        values = []
        for f in fs:
            values.append(args[required.index(f)] if f.get('required') else default(t,f,enums))
        values.append('Map.empty')
        lines += [factory+' '+ ' '.join(args) + ' = '+n+'Fields '+' '.join('('+v+')' for v in values),
                  f'-- | Inspect immutable construction fields. Rebuild edits with \'build{n}\' to revalidate.', f'inspect{n} :: {n} -> {n}Fields',f'inspect{n} (Checked{n} fields) = fields',
                  f'-- | Validate fields, extensions and receiving context. Return structured errors or a checked {n}.', f'build{n} :: {n}Fields -> Result {n}',f'build{n} fields = do',
                  f'  checkExtensions {hslist(string(f["wire"]) for f in fs)} ({factory}Extensions fields)',
                  f'  validateFields {string(t["name"])} (wire{n} fields)',f'  pure (Checked{n} fields)',
                  f'wire{n} :: {n}Fields -> Value',f'wire{n} fields = object $']
        pieces = ['['+'"type" .= ('+string(t['wireType'])+' :: Text)'+']'] if t['wireType'] else ['[]']
        for f in fs:
            value = fieldname(t,f)+' fields'
            if t['name']=='PlanBlock' and f['wire']=='tasks': value='map (stripTag . toJSON) ('+value+')'
            if f.get('required'): piece='['+string(f['wire'])+' .= ('+value+')]'
            else: piece='optional '+string(f['wire'])+' ('+value+')'
            pieces.append(piece)
        pieces.append('extensionPairs ('+factory+'Extensions fields)')
        if t['name'] == 'TaskCardBlock':
            position = lines.index('  pure (CheckedTaskCardBlock fields)')
            lines.insert(position, '  standaloneTask (wireTaskCardBlock fields)')
        lines += ['    '+ ' <> '.join(pieces),f'instance ToJSON {n} where toJSON (Checked{n} fields) = wire{n} fields']
        if t['name'] == 'TaskCardBlock':
            lines += ['instance FromSlackJSON TaskCardBlock where fromSlackJSON = decodeTaskCard buildTaskCardBlock',
                      'decodeTaskCard :: (TaskCardBlockFields -> Result a) -> Value -> Result a',
                      'decodeTaskCard constructor value = do']
        else:
            lines += [f'instance FromSlackJSON {n} where', '  fromSlackJSON value = do']
        lines += [f'    fields <- wireObject {string(t["wireType"])} value']
        if any(f['kind'] in {'int', 'long'} and f.get('limits') for f in fs):
            lines += [f'    checkIntegerBounds {string(t["name"])} (Object fields)']
        for f in fs:
            method = 'required' if f.get('required') else 'optionalField'
            expression = f'{method} {string(f["wire"])} fields'
            if f['kind'] in {'text','textList'} or f.get('coerce') == 'decimal_string':
                expression = f'{method} {string(f["wire"])} (coerceField {string(f["wire"])} {string(f.get("coerce","plain_text"))} fields)'
            if t['name']=='PlanBlock' and f['wire']=='tasks':
                expression = f'{method} "tasks" (restoreTasks fields)'
            lines += [f'    parsed{f["method"][:1].upper()+f["method"][1:]} <- {expression}']
        builder = 'constructor' if t['name'] == 'TaskCardBlock' else 'build'+n
        lines += [f'    {builder} ({n}Fields '+' '.join('parsed'+f['method'][:1].upper()+f['method'][1:] for f in fs)+' (remaining '+hslist(string(f['wire']) for f in fs)+' fields))',
                  f'instance FromJSON {n} where parseJSON = checkedParser','']
    exports += ['PlanTask', 'buildPlanTask', 'inspectPlanTask']
    task_fields = next(t['fields'] for t in model['types'] if t['name'] == 'TaskCardBlock')
    lines += ['-- | A task in a plan. Unlike a standalone task card, it may be pending.',
              'newtype PlanTask = CheckedPlanTask TaskCardBlock deriving (Eq, Show)',
              '-- | Check a task for a plan, where pending is allowed. This value cannot enter the Block role.', 'buildPlanTask :: TaskCardBlockFields -> Result PlanTask',
              'buildPlanTask fields = do',
              '  checkExtensions '+hslist(string(f['wire']) for f in task_fields)+' (taskCardBlockExtensions fields)',
              '  validateFields "TaskCardBlock" (wireTaskCardBlock fields)',
              '  pure (CheckedPlanTask (CheckedTaskCardBlock fields))',
              '-- | Inspect a plan task; call buildPlanTask after editing its fields.', 'inspectPlanTask :: PlanTask -> TaskCardBlockFields',
              'inspectPlanTask (CheckedPlanTask task) = inspectTaskCardBlock task',
              'instance ToJSON PlanTask where toJSON (CheckedPlanTask task) = stripTag (toJSON task)',
              'instance FromSlackJSON PlanTask where fromSlackJSON = decodeTaskCard buildPlanTask . restoreTask',
              'instance FromJSON PlanTask where parseJSON = checkedParser', '']
    for role, members in roles.items():
        n=name(role); exports.append(n)
        lines += ['-- | Closed role: only checked members may enter this position.',f'data {n} = '+' | '.join(n+name(t['name'])+' '+name(t['name']) for t in members)+' deriving (Eq, Show)',f'instance ToJSON {n} where']
        lines += ['  toJSON ('+n+name(t['name'])+' value) = toJSON value' for t in members]
        lines += [f'instance FromSlackJSON {n} where','  fromSlackJSON value = do','    tag <- wireTag value','    case tag of']
        for t in members:
            tn=name(t['name']); fn=lower(tn)+'As'+role
            check = role=='Block' and t['name']=='TaskCardBlock'
            lines += ['      '+string(t['wireType'])+' -> '+ ('fromSlackJSON value >>= '+fn if check else n+tn+' <$> fromSlackJSON value')]
        lines += ['      _ -> failure TypeMismatch [Field "type"] "role" "unsupported child type"',f'instance FromJSON {n} where parseJSON = checkedParser']
        for t in members:
            tn=name(t['name']); fn=lower(tn)+'As'+role; match='match'+n+tn
            exports += [fn,match]
            lines += [f'-- | Place a checked {tn} in the {n} role. The receiving parent retains its contextual validation.']
            if role=='Block' and t['name']=='TaskCardBlock':
                lines += [fn+' :: '+tn+' -> Result '+n,fn+' value = do','  standaloneTask (toJSON value)','  pure ('+n+tn+' value)']
            else: lines += [fn+' :: '+tn+' -> '+n,fn+' = '+n+tn]
            lines += [f'-- | Return the {tn} member when present, otherwise Nothing; no unchecked cast.', match+' :: '+n+' -> Maybe '+tn,match+' ('+n+tn+' value) = Just value']
            if len(members)>1: lines.append(match+' _ = Nothing')
        lines.append('')
    header = ['-- Generated by haskell/generator/generate.py. Edit the shared contract instead.', '-- | Native checked models, construction fields and closed role conversions.', 'module Slackblocks.Models', '  ( '+ '\n  , '.join(exports)+'\n  ) where',
              'import Data.Aeson (FromJSON(..), ToJSON(..), Value(..), object, (.=))','import Data.Map.Strict (Map)','import qualified Data.Map.Strict as Map',
              'import Data.Scientific (Scientific)','import Data.Text (Text)','import Slackblocks.Validation','import Slackblocks.Json (FromSlackJSON(..))','import Slackblocks.Internal.Wire',
              'import Slackblocks.Internal.Rules (validateFields, checkIntegerBounds, standaloneTask)','import Slackblocks.Style',
              '-- | Coordinated package version, distinct from the shared contract.', 'packageVersion :: Text', '-- | Shared Block Kit contract version implemented by this package.', 'specVersion :: Text','packageVersion = "3.2.0"', 'specVersion = '+string(json.loads((ROOT/'spec/manifest.json').read_text(encoding="utf-8"))['spec_version']),'']
    return '\n'.join(header+lines)+'\n'


def limits_flat(values, prefix=''):
    out=[]
    for k,v in values.items():
        key=prefix+'.'+k if prefix else k
        if isinstance(v,dict): out.extend(limits_flat(v,key))
        elif type(v) is int: out.append((key,v))
        else: raise ValueError('Non-integral limit '+key)
    return out


def schema(model, limits, vocab):
    lines=['-- Generated shared limits, vocabulary and local field constraints.', 'module Slackblocks.Internal.Schema (fieldConstraints, limits, icons, surfaceBlocks) where',
           'import Data.Text (Text)', 'fieldConstraints :: Text -> [(Text, Text, Text, [(Text, Integer)], [Text])]', 'fieldConstraints owner = case owner of']
    for t in model['types']:
        constraints=[]
        for f in t['fields']:
            prefix=f.get('limits','')
            if t['name'] in {'PlainText','MarkdownText'} and f['wire']=='text': prefix='text'
            bounds=limits
            if prefix:
                for part in prefix.split('.'): bounds=bounds[part]
                supported = {'min_length', 'max_length', 'min_items', 'max_items', 'min', 'max', 'exclusive_min', 'exclusive_max', 'item_max_length'}
                if any(type(v) is not int or k not in supported for k,v in bounds.items()): raise ValueError('Invalid bound '+prefix)
            else: bounds={}
            flags=f.get('flags',[])
            if bounds or flags:
                constraints.append('('+', '.join([string(f['wire']),string(f['kind']),string(prefix),hslist('('+string(k)+', '+str(v)+')' for k,v in bounds.items()),hslist(string(x) for x in flags)])+')')
        lines.append('  '+string(t['name'])+' -> '+hslist(constraints))
    lines += ['  _ -> []','limits :: [(Text, Integer)]','limits = '+hslist('('+string(k)+', '+str(v)+')' for k,v in limits_flat(limits)),
              'icons :: [Text]','icons = '+hslist(string(x) for x in vocab['slack_icon_names']), 'surfaceBlocks :: Text -> [Text]','surfaceBlocks surface = case surface of']
    lines += ['  '+string(k)+' -> '+hslist(string(v) for v in vs) for k,vs in vocab['surface_block_types'].items()]
    lines += ['  _ -> []']
    return '\n'.join(lines)+'\n'


def main():
    parser=argparse.ArgumentParser();parser.add_argument('--check',action='store_true');args=parser.parse_args()
    model=json.loads((ROOT/'spec/model.json').read_text(encoding="utf-8"));limits=json.loads((ROOT/'spec/limits.json').read_text(encoding="utf-8"));vocab=json.loads((ROOT/'spec/vocabulary.json').read_text(encoding="utf-8"))
    outputs={PACKAGE/'src/Slackblocks/Models.hs':generate(model),PACKAGE/'src/Slackblocks/Internal/Schema.hs':schema(model,limits,vocab)}
    for path in (PACKAGE/'src').rglob('*.hs'):
        if path not in outputs and path.read_text(encoding="utf-8").startswith(('-- Generated by haskell/generator/', '-- Generated shared limits')):
            raise SystemExit('Obsolete generated output: '+str(path))
    stale=[]
    version = subprocess.check_output(['ormolu', '--version'], text=True)
    if version.splitlines()[0] != 'ormolu 0.7.7.0':
        raise SystemExit('Generation requires ormolu 0.7.7.0')
    for path,value in outputs.items():
        value = subprocess.run(['ormolu', '--stdin-input-file', str(path)], input=value,
                               text=True, capture_output=True, check=True).stdout
        if args.check:
            if not path.is_file() or path.read_text(encoding="utf-8")!=value: stale.append(str(path.relative_to(ROOT)))
        else:
            path.parent.mkdir(parents=True,exist_ok=True);path.write_text(value, encoding="utf-8")
    if stale: raise SystemExit('Generated output differs: '+', '.join(stale))


if __name__=='__main__': main()
