package slackblocks_test

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"reflect"
	"regexp"
	"strings"
	"testing"
)

// Higher-level components are documented with several examples and no single JSON tab; their
// payloads are covered by components_test.go.
var usingBlocksSkippedSections = map[string]bool{"Higher-Level Components": true}

var (
	headingPattern   = regexp.MustCompile(`(?m)^## (.+)$`)
	goSectionPattern = regexp.MustCompile(`(?s)<Go>(.*?)</Go>`)
	goFencePattern   = regexp.MustCompile("(?s)```go\n(.*?)```")
	jsonFencePattern = regexp.MustCompile("(?s)```json\n(.*?)```")
	snippetPattern   = regexp.MustCompile(`(?m)^block, err := slackblocks\.`)
)

type usingBlocksSnippet struct {
	title    string
	code     string
	expected string
}

// TestUsingBlocksSnippetsMatchDocumentedJSON runs every Go snippet on the "Using Blocks" guide
// and checks that it produces the JSON shown beside it, so the guide cannot drift from the API.
func TestUsingBlocksSnippetsMatchDocumentedJSON(t *testing.T) {
	guide, err := os.ReadFile(filepath.Join("..", "docs", "docs", "usage", "using_blocks.mdx"))
	if err != nil {
		t.Fatal(err)
	}
	snippets := usingBlocksSnippets(t, string(guide))
	if len(snippets) < 21 {
		t.Fatalf("expected a Go snippet with a JSON tab for every block section, found %d", len(snippets))
	}

	var program bytes.Buffer
	program.WriteString("package main\n\nimport (\n\t\"encoding/json\"\n\t\"os\"\n\n\tslackblocks \"github.com/nicklambourne/slackblocks/go/v2\"\n)\n\n")
	for index, snippet := range snippets {
		fmt.Fprintf(&program, "func snippet%d() (any, error) {\n%s\n\treturn block, err\n}\n\n", index, snippet.code)
	}
	program.WriteString("func main() {\n\tresults := []any{}\n")
	for index := range snippets {
		fmt.Fprintf(&program, "\tif block, err := snippet%d(); err != nil {\n\t\tresults = append(results, map[string]any{\"error\": err.Error()})\n\t} else {\n\t\tresults = append(results, block)\n\t}\n", index)
	}
	program.WriteString("\tif err := json.NewEncoder(os.Stdout).Encode(results); err != nil {\n\t\tpanic(err)\n\t}\n}\n")

	// go run needs the program inside the module, so it is written to a temporary package.
	directory, err := os.MkdirTemp(filepath.Join("internal"), "docsnippets-")
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { os.RemoveAll(directory) })
	if err := os.WriteFile(filepath.Join(directory, "main.go"), program.Bytes(), 0o644); err != nil {
		t.Fatal(err)
	}
	command := exec.Command("go", "run", "./"+directory)
	output, err := command.CombinedOutput()
	if err != nil {
		t.Fatalf("Using Blocks Go snippets do not compile or run: %v\n%s", err, output)
	}
	var results []any
	if err := json.Unmarshal(output, &results); err != nil {
		t.Fatalf("snippet program output is not JSON: %v\n%s", err, output)
	}

	for index, snippet := range snippets {
		t.Run(snippet.title, func(t *testing.T) {
			var expected any
			if err := json.Unmarshal([]byte(snippet.expected), &expected); err != nil {
				t.Fatalf("documented JSON is invalid: %v", err)
			}
			if !reflect.DeepEqual(results[index], expected) {
				actual, _ := json.Marshal(results[index])
				t.Fatalf("Go snippet differs from the documented JSON\nwant: %s\n got: %s", strings.TrimSpace(snippet.expected), actual)
			}
		})
	}
}

func usingBlocksSnippets(t *testing.T, guide string) []usingBlocksSnippet {
	t.Helper()
	headings := headingPattern.FindAllStringSubmatchIndex(guide, -1)
	snippets := []usingBlocksSnippet{}
	for index, heading := range headings {
		title := guide[heading[2]:heading[3]]
		end := len(guide)
		if index+1 < len(headings) {
			end = headings[index+1][0]
		}
		section := guide[heading[1]:end]
		goSection := goSectionPattern.FindStringSubmatch(section)
		expected := jsonFencePattern.FindStringSubmatch(section)
		if goSection == nil || expected == nil {
			continue
		}
		fence := goFencePattern.FindStringSubmatch(goSection[1])
		if fence == nil || !snippetPattern.MatchString(fence[1]) {
			if !usingBlocksSkippedSections[title] {
				t.Errorf("section %q has no Go snippet assigning block, err", title)
			}
			continue
		}
		snippets = append(snippets, usingBlocksSnippet{title: title, code: fence[1], expected: expected[1]})
	}
	return snippets
}
