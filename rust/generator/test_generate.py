"""Tests generation failures rather than accepting incomplete generated output."""
import copy
import importlib.util
import json
from pathlib import Path
import unittest

HERE=Path(__file__).parent
spec=importlib.util.spec_from_file_location('generate',HERE/'generate.py')
generate=importlib.util.module_from_spec(spec)
spec.loader.exec_module(generate)

class GeneratorTests(unittest.TestCase):
    def setUp(self):self.model=json.loads((HERE.parents[1]/'spec/model.json').read_text())
    def test_unknown_kind_fails(self):
        self.model['types'][0]['fields'][0]['kind']='future_kind'
        with self.assertRaisesRegex(ValueError,'Unknown kind'):generate.resolve(self.model)
    def test_unresolved_type_fails(self):
        self.model['types'][0]['fields'][0]['type']='Missing'
        with self.assertRaisesRegex(ValueError,'Unresolved type'):generate.resolve(self.model)
    def test_wire_collision_fails(self):
        fields=self.model['types'][0]['fields'];fields.append(copy.deepcopy(fields[0]))
        with self.assertRaisesRegex(ValueError,'Duplicate field'):generate.resolve(self.model)
    def test_reserved_name_fails(self):
        self.model['types'][0]['fields'][0]['wire']='build'
        with self.assertRaisesRegex(ValueError,'colliding'):generate.resolve(self.model)
    def test_invalid_default_fails(self):
        t=next(t for t in self.model['types'] if t['name']=='IconButtonElement');t['defaults']['icon']='unknown'
        with self.assertRaisesRegex(ValueError,'Invalid enum default'):generate.resolve(self.model)
    def test_role_membership_is_explicit_and_includes_packages(self):
        _,roles=generate.resolve(self.model,full=True)
        self.assertIn('SectionBlock',[t['name'] for t in roles['Block']])
        self.assertIn('PlainTextInputElement',[t['name'] for t in roles['Element']])
    def test_stale_and_missing_outputs_fail(self):
        # Exercise the real command without modifying the checked-out generated files.
        import subprocess, tempfile, shutil, os
        with tempfile.TemporaryDirectory(dir=os.environ.get('TMPDIR')) as tmp:
            root=Path(tmp);shutil.copytree(HERE.parents[1]/'spec',root/'spec');shutil.copytree(HERE.parent,root/'rust',ignore=shutil.ignore_patterns('target','__pycache__'))
            command=['python3',str(root/'rust/generator/generate.py'),'--check']
            subprocess.run(command,check=True,capture_output=True)
            stale=root/'rust/src/generated/obsolete.rs';stale.write_text('// stale')
            result=subprocess.run(command,capture_output=True,text=True)
            self.assertNotEqual(result.returncode,0);self.assertIn('Unexpected generated files',result.stderr)
            stale.unlink();(root/'rust/src/generated/models.rs').unlink()
            result=subprocess.run(command,capture_output=True,text=True)
            self.assertNotEqual(result.returncode,0);self.assertIn('Generated output differs',result.stderr)

if __name__=='__main__':unittest.main()
