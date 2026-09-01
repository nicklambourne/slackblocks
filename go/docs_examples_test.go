package slackblocks_test

import (
	"encoding/json"
	"os"
	"os/exec"
	"path/filepath"
	"reflect"
	"testing"
)

func TestExecutableDocumentationExample(t *testing.T) {
	command := exec.Command("go", "run", filepath.Join("..", "docs", "examples", "go", "section_hello.go"))
	actualData, err := command.Output()
	if err != nil {
		t.Fatal(err)
	}
	expectedData, err := os.ReadFile(filepath.Join("..", "docs", "examples", "section_hello.json"))
	if err != nil {
		t.Fatal(err)
	}
	var actual, expected any
	if err := json.Unmarshal(actualData, &actual); err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal(expectedData, &expected); err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(actual, expected) {
		t.Fatalf("Go docs example differs from shared JSON\nwant: %#v\n got: %#v", expected, actual)
	}
}
