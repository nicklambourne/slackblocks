#!/usr/bin/env python3
"""Install the actual Composer archive outside the checkout, without a path repository."""
import hashlib
import json
import os
import shutil
from pathlib import Path
import subprocess
import tempfile
import zipfile

ROOT = Path(__file__).resolve().parents[1]

def run(*cmd, cwd=ROOT, env=None):
    executable = shutil.which(cmd[0])
    if executable is None: raise SystemExit('Required tool not found: '+cmd[0])
    subprocess.run([executable, *cmd[1:]], cwd=cwd, check=True, env=env)

with tempfile.TemporaryDirectory(prefix='slackblocks-php-package-') as directory:
    work = Path(directory)
    run('composer','archive','--format=zip','--file=slackblocks','--dir='+str(work))
    archive = work/'slackblocks.zip'
    with zipfile.ZipFile(archive) as package:
        names=package.namelist()
        allowed={'src','examples','composer.json','README.md','IMPLEMENTATION.md','CHANGELOG.md','LICENSE','LICENSE.BSD-3-Clause'}
        unexpected=[p for p in names if p.split('/')[0] not in allowed or '..' in Path(p).parts or p.startswith('/')]
        if unexpected:raise SystemExit('Unexpected package contents: '+repr(unexpected[:12])+f' ({len(unexpected)} total)')
        manifest=json.loads(package.read('composer.json'))
        required=['src/MessagePayload.php','src/Internal/Schema.php','src/Builder.php','LICENSE','README.md']
        if not set(required)<=set(names):raise SystemExit('Missing package runtime/license/docs files')
    consumer=work/'consumer';consumer.mkdir()
    # A Composer package repository supplies registry metadata for the unchanged ZIP.
    # Its dist URL points to the tested artifact, never the source checkout.
    metadata={k:v for k,v in manifest.items() if k not in ['require-dev','autoload-dev','scripts','config','archive']}
    metadata.update(version='2.7.0',dist={'type':'zip','url':archive.as_uri(),'shasum':hashlib.sha1(archive.read_bytes()).hexdigest()})
    (consumer/'composer.json').write_text(json.dumps({'repositories':[{'type':'package','package':metadata}],'require':{manifest['name']:'2.7.0'},'config':{'allow-plugins':False}}))
    run('composer','install','--no-interaction','--prefer-dist','--no-scripts',cwd=consumer)
    autoloader=consumer/'vendor/autoload.php'
    installed=consumer/'vendor'/manifest['name']
    if installed.is_symlink():raise SystemExit('Package evidence must not use a checkout symlink')
    run('php',str(installed/'examples/consumer.php'),str(autoloader),cwd=consumer)
    # Standalone autoload must not resolve any library class from the monorepo.
    script=consumer/'location.php'
    script.write_text("<?php require __DIR__.'/vendor/autoload.php'; echo (new ReflectionClass(Slackblocks\\MessagePayload::class))->getFileName();")
    location=subprocess.check_output(['php',str(script)],cwd=consumer,text=True)
    if not Path(location).resolve().is_relative_to(installed.resolve()):raise SystemExit('Consumer loaded the source checkout')
    print(f'Archive verified: {len(names)} entries; SHA256 {hashlib.sha256(archive.read_bytes()).hexdigest()}')
