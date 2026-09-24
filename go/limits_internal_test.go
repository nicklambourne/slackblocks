package slackblocks

import (
	"encoding/json"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestGeneratedLimitsMatchSharedLimits(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "spec", "limits.json"))
	if err != nil {
		t.Fatal(err)
	}
	var limits map[string]any
	if err := json.Unmarshal(data, &limits); err != nil {
		t.Fatal(err)
	}
	want := map[string]string{}
	flattenSharedLimits("", limits, want)

	file, err := parser.ParseFile(token.NewFileSet(), "limits_gen.go", nil, parser.ParseComments)
	if err != nil {
		t.Fatal(err)
	}
	got := map[string]string{}
	for _, declaration := range file.Decls {
		group, ok := declaration.(*ast.GenDecl)
		if !ok || group.Tok != token.CONST {
			continue
		}
		for _, raw := range group.Specs {
			spec := raw.(*ast.ValueSpec)
			value, ok := spec.Values[0].(*ast.BasicLit)
			if !ok || spec.Comment == nil {
				t.Fatalf("%s is not a literal limit annotated with its spec/limits.json path", spec.Names[0].Name)
			}
			got[strings.TrimSpace(spec.Comment.Text())] = value.Value
		}
	}
	for path, value := range want {
		if got[path] != value {
			t.Errorf("%s is %s in spec/limits.json but %q in limits_gen.go; run go generate ./...", path, value, got[path])
		}
	}
	if len(got) != len(want) {
		t.Errorf("limits_gen.go has %d limits, spec/limits.json has %d; run go generate ./...", len(got), len(want))
	}
}

func flattenSharedLimits(prefix string, node map[string]any, leaves map[string]string) {
	for key, value := range node {
		path := strings.TrimPrefix(prefix+"."+key, ".")
		if nested, ok := value.(map[string]any); ok {
			flattenSharedLimits(path, nested, leaves)
			continue
		}
		encoded, _ := json.Marshal(value)
		leaves[path] = string(encoded)
	}
}
