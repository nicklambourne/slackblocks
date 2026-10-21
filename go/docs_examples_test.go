package slackblocks_test

import (
	"encoding/json"
	"os"
	"os/exec"
	"path/filepath"
	"reflect"
	"strings"
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

func TestResponseURLDocumentationExample(t *testing.T) {
	example := filepath.Join("..", "docs", "examples", "go", "response_url_payloads.go")
	source, err := os.ReadFile(example)
	if err != nil {
		t.Fatal(err)
	}
	guide, err := os.ReadFile(filepath.Join("..", "docs", "docs", "usage", "sending_messages.mdx"))
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(string(guide), "```go title=\"docs/examples/go/response_url_payloads.go\"\n"+strings.TrimSpace(string(source))+"\n```") {
		t.Fatal("response URL recipe differs from executable example")
	}
	actualData, err := exec.Command("go", "run", example).Output()
	if err != nil {
		t.Fatal(err)
	}
	expectedData, err := os.ReadFile(filepath.Join("..", "docs", "examples", "response_url_payloads.json"))
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
		t.Fatalf("response URL recipe wire mismatch: %s", actualData)
	}
}
