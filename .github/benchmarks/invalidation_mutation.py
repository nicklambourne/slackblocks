"""Exactly one requested invalidation input per full-site control."""
from pathlib import Path
import hashlib,json

def mutate(work,pair):
    cases={1:('docs/docs/index.mdx','WebpackSourceInvalidation20261021'),2:('docs/docusaurus.config.ts','WebpackConfigInvalidation20261021'),3:('docs/src/css/custom.css','WebpackCssInvalidation20261021')}
    name,marker=cases[pair];path=work/name;before=path.read_bytes();text=before.decode();assert marker not in text
    if pair==1:text+='\n\n'+marker+' verifies fresh documentation source.\n'
    elif pair==2:
        original='copyright: `Copyright © ${new Date().getFullYear()}'
        assert text.count(original)==1
        text=text.replace(original,'copyright: `'+marker+' Copyright © ${new Date().getFullYear()}')
    else:text+='\n:root { --webpack-cache-invalidation: "'+marker+'"; }\n'
    path.write_text(text)
    return {'pair':pair,'path':name,'marker':marker,'original_sha256':hashlib.sha256(before).hexdigest(),'changed_sha256':hashlib.sha256(path.read_bytes()).hexdigest()}

def verify_marker(work,control,dest):
    suffix='.css' if control['pair']==3 else '.html'
    files=[str(p.relative_to(work/'docs/build')) for p in (work/'docs/build').rglob('*'+suffix) if control['marker'] in p.read_text()]
    assert files, 'Changed source/config/CSS marker absent: restored cache may be stale'
    (dest/'invalidation-marker.json').write_text(json.dumps({'marker':control['marker'],'files':files},indent=2)+'\n')
