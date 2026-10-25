#!/usr/bin/env python3
"""Render public GHC signatures and compiled Haddock comments, with export parity."""
import html
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[2]
PACKAGE = ROOT / 'haskell'
DOMAINS = {'blocks': 'Blocks', 'elements': 'Elements', 'objects': 'Composition objects and rich text',
           'payloads': 'Messages, attachments and views', 'components': 'Components and utilities',
           'core': 'JSON and public types', 'errors': 'Validation errors'}
SYMBOL = re.compile(r'^(?:type )?(Slackblocks(?:\.[A-Za-z][A-Za-z0-9_\']*)+)\s*::', re.M)


def anchor(name):
    return name.lower().replace('.', '-')


def safe(text):
    return html.escape(text, quote=False).replace('{', '&#123;').replace('}', '&#125;')


def qualified(name):
    return '.'.join(name.split('$')[-2:])


def prose(doc, links):
    tag = doc['tag']
    if tag == 'DocEmpty': return ''
    if tag == 'DocString': return safe(doc['string'])
    if tag == 'DocAppend': return prose(doc['first'], links) + prose(doc['second'], links)
    if tag == 'DocParagraph': return prose(doc['document'], links) + '\n\n'
    if tag in ('DocMonospaced', 'DocEmphasis', 'DocBold'):
        delimiter = {'DocMonospaced': '`', 'DocEmphasis': '*', 'DocBold': '**'}[tag]
        return delimiter + prose(doc['document'], links).strip() + delimiter
    if tag == 'DocIdentifier':
        name = qualified(doc['name'])
        return f'[`{name.rsplit(".", 1)[-1]}`]({links[name]})' if name in links else '`' + name + '`'
    if tag == 'DocHyperlink':
        link = doc['hyperlink']
        label = prose(link['hyperlinkLabel'], links) if link['hyperlinkLabel'] else link['hyperlinkUrl']
        return f'[{label}]({link["hyperlinkUrl"]})'
    raise ValueError(f'Unhandled native Haddock document: {doc!r}')


def main():
    build = os.environ.get('SLACKBLOCKS_HASKELL_BUILD', 'dist-docs')
    cabal = ['cabal', f'--builddir={build}']
    subprocess.run([*cabal, 'haddock', 'lib:slackblocks', '--haddock-html'], cwd=PACKAGE, check=True)
    # Cabal list-bin does not name a library's Haddock interface. Restrict discovery
    # to this build tree, package version and actual compiler's output.
    version = re.search(r'^version:\s*(\S+)', (PACKAGE / 'slackblocks.cabal').read_text(), re.M)[1]
    compiler = subprocess.check_output(['ghc', '--numeric-version'], text=True).strip()
    tree = Path(build) if Path(build).is_absolute() else PACKAGE / build
    interfaces = list(tree.glob(f'build/*/ghc-{compiler}/slackblocks-{version}/doc/html/slackblocks/slackblocks.haddock'))
    if len(interfaces) != 1: raise ValueError(f'Expected one compiled interface, got {interfaces}')
    native = subprocess.run(['haddock', '--show-interface=' + str(interfaces[0])], text=True, capture_output=True, check=True)
    interface = json.loads((native.stdout + native.stderr).strip())
    inventory = json.loads((PACKAGE / 'conformance/public-api.json').read_text())
    commands = ':module Prelude\n' + ''.join(f'putStrLn "REFERENCE_MODULE:{module}"\n:browse! {module}\n' for module in inventory['exports']) + ':quit\n'
    result = subprocess.run([*cabal, 'exec', '--', 'ghci', '-ignore-dot-ghci', '-v0', '-package', 'slackblocks'], cwd=PACKAGE, input=commands, text=True, capture_output=True, check=True)
    if 'error:' in result.stderr.lower(): raise ValueError(result.stderr)
    chunks = re.split(r'^REFERENCE_MODULE:(\S+)\s*$', result.stdout, flags=re.M)
    signatures = {}
    for index in range(1, len(chunks), 2):
        module, source = chunks[index:index+2]
        matches = list(SYMBOL.finditer(source))
        actual = {match[1] for match in matches}
        if actual != set(inventory['exports'][module]): raise ValueError(f'Compiled exports differ: {module}')
        for match in matches:
            lines = source[match.start():].splitlines()
            signature = [lines[0]]
            for line in lines[1:]:
                if not line.startswith(' '): break
                signature.append(line)
            value = '\n'.join(signature)
            if value not in signatures.setdefault(match[1], []): signatures[match[1]].append(value)
    comments = {}
    for module in interface['inst_ifaces']:
        if module['module'].split('$')[-1] not in inventory['exports']: continue
        for name, doc in module['doc_map'].items(): comments[qualified(name)] = doc['document']
    missing = sorted(set(signatures) - set(comments))
    if missing: raise ValueError('Public native comments missing: ' + ', '.join(missing))
    models = json.loads((ROOT / 'spec/model.json').read_text())['types']
    model_domains = {}
    for model in models:
        native_name = inventory['models'][model['name']]['type'].rsplit('.', 1)[-1]
        group = {'block': 'blocks', 'element': 'elements', 'object': 'objects', 'richtext': 'objects', 'view': 'payloads', 'message': 'payloads'}.get(model['package'], 'payloads')
        model_domains[native_name] = group
    def domain(symbol):
        module, name = symbol.rsplit('.', 1)
        if module.endswith('.Helpers'): return 'components'
        if module.endswith('.Validation'): return 'errors'
        if module.endswith('.Style'): return 'objects'
        for model in sorted(model_domains, key=len, reverse=True):
            if name.startswith((model, model[0].lower() + model[1:], 'build' + model, 'inspect' + model)):
                return model_domains[model]
        return 'core'
    links = {name: f'/reference/haskell/{domain(name)}#{anchor(name)}' for name in signatures}
    entries = [{'name': name, 'domain': domain(name), 'anchor': anchor(name), 'signatures': signatures[name], 'doc': prose(comments[name], links).strip()} for name in sorted(signatures)]
    if any(not entry['doc'] for entry in entries): raise ValueError('Empty native comments')
    directory = ROOT / 'docs/docs/reference/haskell'
    if directory.exists(): shutil.rmtree(directory)
    directory.mkdir(parents=True)
    for position, (slug, title) in enumerate(DOMAINS.items(), 1):
        text = f'---\nsidebar_position: {position}\n---\n\n# {title}\n\nSignatures come from GHC; descriptions come from compiled Haddock comments. Import `Slackblocks` or the defining public module. Checked values encode with Aeson; field records must be built first.\n\n'
        for entry in (entry for entry in entries if entry['domain'] == slug):
            text += f'## {entry["name"].rsplit(".", 1)[-1]} {{#{entry["anchor"]}}}\n\n{entry["doc"]}\n\n```haskell\n' + '\n\n'.join(entry['signatures']) + '\n```\n\n'
        (directory / f'{slug}.mdx').write_text(text, encoding='utf-8')
    (directory / 'index.mdx').write_text('---\nsidebar_position: 0\n---\n\n# Haskell API reference\n\nIntroduced in the upcoming coordinated 3.1.0 release; Hackage publication is not active yet. See [installation](/usage/installation) and the [compiler support table](/usage/compatibility). These pages cover every compiled public type, constructor, field, role conversion and helper.\n\n' + '\n'.join(f'- [{title}](/reference/haskell/{slug})' for slug, title in DOMAINS.items()) + '\n', encoding='utf-8')
    (directory / 'reference.json').write_text(json.dumps({'version': version, 'compiler': compiler, 'exports': inventory['exports'], 'entries': entries}, indent=2) + '\n', encoding='utf-8')
    print(f'Haskell reference: {len(entries)} compiled public symbols with native Haddock comments.')


if __name__ == '__main__': main()
