from pathlib import Path
import hashlib,json,os,subprocess
root=Path(os.environ['DOTNET_INSTALL_DIR']);assert root==Path(os.environ['DOTNET_ROOT'])
sdks=subprocess.check_output([str(root/'dotnet'),'--list-sdks'],text=True);assert [x.split()[0] for x in sdks.splitlines()]==['8.0.426','10.0.402'],sdks
runtimes=subprocess.check_output([str(root/'dotnet'),'--list-runtimes'],text=True)
assert sorted(' '.join(x.split()[:2]) for x in runtimes.splitlines())==sorted(['Microsoft.AspNetCore.App 8.0.32','Microsoft.AspNetCore.App 10.0.13','Microsoft.NETCore.App 8.0.32','Microsoft.NETCore.App 10.0.13']),runtimes
files={}
for area in ['packs','shared','host','sdk']:
 for file in sorted((root/area).rglob('*')):
  if file.is_file() and not file.is_symlink():files[str(file.relative_to(root))]={'bytes':file.stat().st_size,'sha256':hashlib.sha256(file.read_bytes()).hexdigest()}
assert files
Path('runtime-evidence/dotnet-artifacts.json').write_text(json.dumps({'sdk_versions':['8.0.426','10.0.402'],'runtimes':runtimes.replace(str(root),'/DOTNET'),'files':files},indent=2)+'\n')
