"""Fail closed when shared metadata outgrows the native generator."""
import copy
import json
import unittest
import generate


class GeneratorContracts(unittest.TestCase):
    def setUp(self):
        self.model = json.loads((generate.ROOT / "spec/model.json").read_text(encoding="utf-8"))

    def test_every_native_model_resolves(self):
        roles = generate.resolve(self.model)
        self.assertEqual(set(roles), {r['name'] for r in self.model['interfaces']})

    def test_unknown_field_kind(self):
        self.model['types'][0]['fields'][0]['kind'] = 'future-unimplemented-kind'
        with self.assertRaisesRegex(ValueError, 'Unknown field kind'):
            generate.resolve(self.model)

    def test_unknown_referenced_type(self):
        field = next(f for t in self.model['types'] for f in t['fields'] if 'type' in f)
        field['type'] = 'FutureModel'
        with self.assertRaisesRegex(ValueError, 'Unknown field type'):
            generate.resolve(self.model)

    def test_native_type_collision(self):
        self.model['types'].append(copy.deepcopy(self.model['types'][0]))
        with self.assertRaisesRegex(ValueError, 'Type name collision'):
            generate.resolve(self.model)

    def test_native_field_collision(self):
        self.model['types'][0]['fields'].append(copy.deepcopy(self.model['types'][0]['fields'][0]))
        with self.assertRaisesRegex(ValueError, 'Field name collision'):
            generate.resolve(self.model)

    def test_role_discriminator_collision(self):
        blocks = [t for t in self.model['types'] if t['package'] == 'block']
        blocks[1]['wireType'] = blocks[0]['wireType']
        with self.assertRaisesRegex(ValueError, 'Ambiguous role'):
            generate.resolve(self.model)

    def test_enum_constructor_collision(self):
        self.model['enums'][0]['constants'].append(copy.deepcopy(self.model['enums'][0]['constants'][0]))
        with self.assertRaisesRegex(ValueError, 'Enum constructor collision'):
            generate.resolve(self.model)

    def test_new_constraint_cannot_be_silently_ignored(self):
        limits = json.loads((generate.ROOT / 'spec/limits.json').read_text(encoding="utf-8"))
        vocabulary = json.loads((generate.ROOT / 'spec/vocabulary.json').read_text(encoding="utf-8"))
        limits['text']['future_rule'] = 1
        with self.assertRaisesRegex(ValueError, 'Invalid bound'):
            generate.schema(self.model, limits, vocabulary)


if __name__ == '__main__':
    unittest.main()
