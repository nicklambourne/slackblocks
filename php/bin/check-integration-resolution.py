#!/usr/bin/env python3
"""Resolve optional integrations independently, without changing their committed lock."""
import argparse
import json
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT=Path(__file__).resolve().parents[1]
parser=argparse.ArgumentParser();parser.add_argument('mode',choices=['latest','lowest']);args=parser.parse_args()
with tempfile.TemporaryDirectory(prefix='slackblocks-php-integration-') as directory:
    target=Path(directory)
    for name in ['src','tests']:
        shutil.copytree(ROOT/'integrations'/name,target/name)
    shutil.copy(ROOT/'integrations/phpunit.xml',target/'phpunit.xml')
    manifest=json.loads((ROOT/'integrations/composer.json').read_text())
    manifest['repositories'][0]['url']=str(ROOT)
    (target/'composer.json').write_text(json.dumps(manifest,indent=2)+'\n')
    command=['composer','update','--no-interaction','--prefer-stable']
    if args.mode=='lowest':command.append('--prefer-lowest')
    subprocess.run(command,cwd=target,check=True)
    subprocess.run(['php','vendor/bin/phpunit'],cwd=target,check=True)
    packages=json.loads((target/'composer.lock').read_text())['packages']
    for package in packages:
        if package['name'] in ['illuminate/notifications','jolicode/slack-php-api','laravel/slack-notification-channel','symfony/http-client']:
            print(package['name'],package['version'])
