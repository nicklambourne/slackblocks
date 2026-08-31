package slackblocks_test

import (
	"encoding/json"
	"go/ast"
	"go/importer"
	"go/parser"
	"go/token"
	"go/types"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"strings"
	"testing"

	slackblocks "github.com/nicklambourne/slackblocks/go/v2"
)

func TestConcreteBuildersExposeOnlyRegisteredFluentMethods(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("internal", "builder_methods.json"))
	if err != nil {
		t.Fatal(err)
	}
	registry := map[string][]string{}
	if err := json.Unmarshal(data, &registry); err != nil {
		t.Fatal(err)
	}

	set := token.NewFileSet()
	files := []*ast.File{}
	entries, err := os.ReadDir(".")
	if err != nil {
		t.Fatal(err)
	}
	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".go") || strings.HasSuffix(entry.Name(), "_test.go") {
			continue
		}
		file, err := parser.ParseFile(set, entry.Name(), nil, 0)
		if err != nil {
			t.Fatal(err)
		}
		files = append(files, file)
	}
	checked, err := (&types.Config{Importer: newModuleAwareImporter()}).Check("github.com/nicklambourne/slackblocks/go/v2", set, files, nil)
	if err != nil {
		t.Fatal(err)
	}

	terminal := map[string]bool{
		"BlockType":   true,
		"Build":       true,
		"ID":          true,
		"MarshalJSON": true,
		"MustBuild":   true,
		"Set":         true,
	}
	for constructor, expected := range registry {
		object := checked.Scope().Lookup(constructor)
		function, ok := object.(*types.Func)
		if !ok {
			t.Errorf("%s is not a function", constructor)
			continue
		}
		signature := function.Type().(*types.Signature)
		if signature.Results().Len() != 1 {
			t.Errorf("%s must return one concrete builder", constructor)
			continue
		}
		pointer, ok := signature.Results().At(0).Type().(*types.Pointer)
		if !ok {
			t.Errorf("%s does not return a pointer", constructor)
			continue
		}
		named, ok := pointer.Elem().(*types.Named)
		expectedType := strings.TrimPrefix(constructor, "New") + "Builder"
		if !ok || named.Obj().Name() != expectedType {
			t.Errorf("%s returns %s, want *%s", constructor, signature.Results().At(0).Type(), expectedType)
			continue
		}

		actual := []string{}
		methods := types.NewMethodSet(pointer)
		for index := 0; index < methods.Len(); index++ {
			name := methods.At(index).Obj().Name()
			if ast.IsExported(name) && !terminal[name] {
				actual = append(actual, name)
			}
		}
		sort.Strings(actual)
		want := append([]string(nil), expected...)
		sort.Strings(want)
		if !reflect.DeepEqual(actual, want) {
			t.Errorf("%s methods mismatch\nactual: %v\nwant:   %v", expectedType, actual, want)
		}
	}
}

func TestConcreteBuilderRejectsUnrelatedMethodSets(t *testing.T) {
	section := reflect.TypeOf(slackblocks.NewSectionBlock())
	if _, ok := section.MethodByName("Text"); !ok {
		t.Fatal("SectionBlockBuilder does not expose Text")
	}
	if _, ok := section.MethodByName("ActionID"); ok {
		t.Fatal("SectionBlockBuilder unexpectedly exposes button-only ActionID")
	}

	button := reflect.TypeOf(slackblocks.NewButton())
	if _, ok := button.MethodByName("ActionID"); !ok {
		t.Fatal("ButtonBuilder does not expose ActionID")
	}
	if _, ok := button.MethodByName("Fields"); ok {
		t.Fatal("ButtonBuilder unexpectedly exposes section-only Fields")
	}
}

type moduleAwareImporter struct {
	standard types.Importer
	slack    *types.Package
}

func newModuleAwareImporter() types.Importer {
	// Keep this stub deliberately minimal so the method-set guard does not need
	// to import the compiled dependency. Extend it when production code starts
	// using additional slack-go symbols.
	const path = "github.com/slack-go/slack"
	pkg := types.NewPackage(path, "slack")
	messageBlockTypeName := types.NewTypeName(token.NoPos, pkg, "MessageBlockType", nil)
	messageBlockType := types.NewNamed(messageBlockTypeName, types.Typ[types.String], nil)
	pkg.Scope().Insert(messageBlockTypeName)
	blockMethods := []*types.Func{
		types.NewFunc(token.NoPos, pkg, "BlockType", types.NewSignatureType(nil, nil, nil, nil, types.NewTuple(types.NewVar(token.NoPos, pkg, "", messageBlockType)), false)),
		types.NewFunc(token.NoPos, pkg, "ID", types.NewSignatureType(nil, nil, nil, nil, types.NewTuple(types.NewVar(token.NoPos, pkg, "", types.Typ[types.String])), false)),
	}
	blockName := types.NewTypeName(token.NoPos, pkg, "Block", nil)
	types.NewNamed(blockName, types.NewInterfaceType(blockMethods, nil).Complete(), nil)
	pkg.Scope().Insert(blockName)
	pkg.MarkComplete()
	return moduleAwareImporter{standard: importer.Default(), slack: pkg}
}

func (i moduleAwareImporter) Import(path string) (*types.Package, error) {
	if path == "github.com/slack-go/slack" {
		return i.slack, nil
	}
	return i.standard.Import(path)
}
