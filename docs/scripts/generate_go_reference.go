package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"go/ast"
	"go/parser"
	"go/printer"
	"go/token"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

type domain struct {
	Slug     string
	Title    string
	Position int
	Files    []string
}

type symbol struct {
	Name      string
	Signature string
	Comment   string
	Receiver  string
}

var domains = []domain{
	{Slug: "blocks", Title: "Blocks", Position: 1, Files: []string{"blocks.go"}},
	{Slug: "elements", Title: "Elements", Position: 2, Files: []string{"elements.go"}},
	{Slug: "objects", Title: "Composition Objects", Position: 3, Files: []string{"objects.go"}},
	{Slug: "payloads", Title: "Payloads And Views", Position: 4, Files: []string{"payloads.go"}},
	{Slug: "components", Title: "Components", Position: 5, Files: []string{"components.go"}},
	{Slug: "core", Title: "Core JSON Types", Position: 6, Files: []string{"doc.go", "core.go"}},
	{Slug: "errors", Title: "Validation And Errors", Position: 7, Files: []string{"errors.go", "validation.go"}},
}

var constructorMethods map[string][]string

func main() {
	working, err := os.Getwd()
	must(err)
	repo := filepath.Clean(filepath.Join(working, ".."))
	if filepath.Base(working) != "docs" {
		repo = working
	}
	goRoot := filepath.Join(repo, "go")
	outRoot := filepath.Join(repo, "docs", "docs", "reference", "go")
	must(os.MkdirAll(outRoot, 0o755))

	registryData, err := os.ReadFile(filepath.Join(goRoot, "internal", "builder_methods.json"))
	must(err)
	must(json.Unmarshal(registryData, &constructorMethods))

	builderTypes := map[string]symbol{}
	builderMethods := map[string]map[string]symbol{}
	for _, item := range parseDomain(goRoot, domain{Files: []string{"concrete_builders_gen.go"}}) {
		if item.Receiver == "" {
			builderTypes[item.Name] = item
			continue
		}
		receiver := strings.TrimPrefix(item.Receiver, "*")
		if builderMethods[receiver] == nil {
			builderMethods[receiver] = map[string]symbol{}
		}
		builderMethods[receiver][item.Name] = item
	}

	parsed := map[string][]symbol{}
	typeLinks := map[string]string{}
	for _, group := range domains {
		parsed[group.Slug] = parseDomain(goRoot, group)
		for _, item := range parsed[group.Slug] {
			if item.Receiver == "" && !strings.HasPrefix(item.Name, "New") && item.Name != "Validate" {
				typeLinks[item.Name] = "/reference/go/" + group.Slug + "#" + strings.ToLower(item.Name)
			}
		}
	}

	must(os.WriteFile(filepath.Join(outRoot, "index.mdx"), []byte(indexPage()), 0o644))
	for _, group := range domains {
		page := renderDomain(group, parsed[group.Slug], typeLinks, builderTypes, builderMethods)
		must(os.WriteFile(filepath.Join(outRoot, group.Slug+".mdx"), []byte(page), 0o644))
	}
}

func parseDomain(root string, group domain) []symbol {
	set := token.NewFileSet()
	result := []symbol{}
	for _, name := range group.Files {
		file, err := parser.ParseFile(set, filepath.Join(root, name), nil, parser.ParseComments)
		must(err)
		for _, declaration := range file.Decls {
			switch node := declaration.(type) {
			case *ast.FuncDecl:
				if !node.Name.IsExported() || (name == "validation.go" && node.Name.Name != "Validate") {
					continue
				}
				receiver := ""
				if node.Recv != nil && len(node.Recv.List) > 0 {
					receiver = expression(set, node.Recv.List[0].Type)
					if !ast.IsExported(strings.TrimPrefix(receiver, "*")) {
						continue
					}
				}
				functionCopy := *node
				functionCopy.Doc = nil
				functionCopy.Body = nil
				result = append(result, symbol{Name: node.Name.Name, Signature: expression(set, &functionCopy), Comment: docText(node.Doc), Receiver: receiver})
			case *ast.GenDecl:
				for _, raw := range node.Specs {
					switch spec := raw.(type) {
					case *ast.TypeSpec:
						if node.Tok != token.TYPE || !spec.Name.IsExported() {
							continue
						}
						typeCopy := *spec
						typeCopy.Doc = nil
						typeCopy.Comment = nil
						comment := docText(spec.Doc)
						if comment == "" {
							comment = docText(node.Doc)
						}
						signature := "type " + expression(set, &typeCopy)
						if structure, ok := spec.Type.(*ast.StructType); ok {
							exported := 0
							for _, field := range structure.Fields.List {
								for _, fieldName := range field.Names {
									if fieldName.IsExported() {
										exported++
									}
								}
							}
							if exported == 0 {
								signature = "type " + spec.Name.Name + " struct { /* contains filtered or unexported fields */ }"
							}
						}
						result = append(result, symbol{Name: spec.Name.Name, Signature: signature, Comment: comment})
					case *ast.ValueSpec:
						if node.Tok != token.CONST || len(spec.Names) != 1 || !spec.Names[0].IsExported() {
							continue
						}
						valueCopy := *spec
						valueCopy.Doc = nil
						valueCopy.Comment = nil
						comment := docText(spec.Doc)
						if comment == "" {
							comment = docText(node.Doc)
						}
						result = append(result, symbol{Name: spec.Names[0].Name, Signature: "const " + expression(set, &valueCopy), Comment: comment})
					}
				}
			}
		}
	}
	sort.Slice(result, func(left, right int) bool {
		if result[left].Receiver != result[right].Receiver {
			return result[left].Receiver < result[right].Receiver
		}
		return result[left].Name < result[right].Name
	})
	return result
}

func renderDomain(group domain, symbols []symbol, links map[string]string, builderTypes map[string]symbol, builderMethods map[string]map[string]symbol) string {
	var output strings.Builder
	fmt.Fprintf(&output, "---\nsidebar_position: %d\ntoc_max_heading_level: 2\n---\n\n# %s\n\n", group.Position, group.Title)
	fmt.Fprintf(&output, "This page documents the public Go API for %s. Every constructor and method shown here is part of the fluent `github.com/nicklambourne/slackblocks/go/v2` package.\n\n", strings.ToLower(group.Title))
	if group.Slug == "blocks" {
		output.WriteString("Every block builder implements `slack.Block`: pass it directly to `slack.MsgOptionBlocks` while keeping the channel and other message options in slack-go. Validation runs when slack-go marshals the block, before it sends the request.\n\n")
	}
	if group.Slug == "components" {
		output.WriteString("Higher-level components expose `SlackBlocks() ([]slack.Block, error)`, expanding them into ordinary blocks ready for `slack.MsgOptionBlocks`.\n\n")
	}
	for _, item := range symbols {
		heading := item.Name
		if item.Receiver != "" {
			heading = strings.TrimPrefix(item.Receiver, "*") + "." + item.Name
		}
		fmt.Fprintf(&output, "## %s\n\n", heading)
		if item.Comment != "" {
			fmt.Fprintf(&output, "%s\n\n", item.Comment)
		}
		fmt.Fprintf(&output, "```go\n%s\n```\n\n", item.Signature)
		related := relatedTypes(item.Signature, item.Name, links)
		if len(related) > 0 {
			output.WriteString("Related types: ")
			for index, name := range related {
				if index > 0 {
					output.WriteString(", ")
				}
				fmt.Fprintf(&output, "[`%s`](%s)", name, links[name])
			}
			output.WriteString(".\n\n")
		}
		renderConcreteBuilder(&output, item.Name, builderTypes, builderMethods, group.Slug == "blocks")
	}
	return strings.TrimRight(output.String(), "\n") + "\n"
}

func renderConcreteBuilder(output *strings.Builder, constructor string, builderTypes map[string]symbol, builderMethods map[string]map[string]symbol, isBlock bool) {
	methodNames := constructorMethods[constructor]
	if len(methodNames) == 0 {
		return
	}
	typeName := strings.TrimPrefix(constructor, "New") + "Builder"
	builderType, ok := builderTypes[typeName]
	if !ok {
		panic(fmt.Sprintf("%s returns undocumented %s", constructor, typeName))
	}

	fmt.Fprintf(output, "### %s\n\n", typeName)
	if builderType.Comment != "" {
		fmt.Fprintf(output, "%s Use %s rather than constructing this type directly.\n\n", builderType.Comment, constructor)
	}
	fmt.Fprintf(output, "```go\n%s\n```\n\n", builderType.Signature)
	output.WriteString("Its fluent methods return the same concrete builder, so invalid fields are rejected at compile time.\n\n")
	if isBlock {
		output.WriteString("It also implements `slack.Block`, including `BlockType()` and `ID()`, and can be passed directly to `slack.MsgOptionBlocks`.\n\n")
	}

	for _, methodName := range methodNames {
		method, ok := builderMethods[typeName][methodName]
		if !ok {
			panic(fmt.Sprintf("%s is missing %s.%s", constructor, typeName, methodName))
		}
		fmt.Fprintf(output, "#### %s.%s\n\n", typeName, methodName)
		if method.Comment != "" {
			fmt.Fprintf(output, "%s\n\n", method.Comment)
		}
		fmt.Fprintf(output, "```go\n%s\n```\n\n", method.Signature)
	}

	fmt.Fprintf(output, "Every `%s` also provides `Build() (Object, error)`, `MustBuild() Object`, and JSON marshaling. `Set(field, value)` is available as an advanced raw wire-format escape hatch; it deliberately ends the typed fluent chain.\n\n", typeName)
}

func relatedTypes(signature, ownName string, links map[string]string) []string {
	names := []string{}
	for name := range links {
		if name != ownName && containsIdentifier(signature, name) {
			names = append(names, name)
		}
	}
	sort.Strings(names)
	return names
}

func containsIdentifier(value, name string) bool {
	for index := 0; ; {
		found := strings.Index(value[index:], name)
		if found < 0 {
			return false
		}
		found += index
		leftOK := found == 0 || !isIdentifier(value[found-1])
		right := found + len(name)
		rightOK := right == len(value) || !isIdentifier(value[right])
		if leftOK && rightOK {
			return true
		}
		index = right
	}
}

func isIdentifier(value byte) bool {
	return value == '_' || value >= '0' && value <= '9' || value >= 'A' && value <= 'Z' || value >= 'a' && value <= 'z'
}

func indexPage() string {
	return `---
sidebar_position: 0
---

# Go API reference

This is the complete guide to slackblocks' public Go API. Use it to check concrete fluent constructor and method signatures, returned JSON types, validation failures, and the higher-level components available for building a message, modal, or App Home tab.

Most slack-go programs pass builders such as ` + "`NewSectionBlock()`" + ` directly to ` + "`slack.MsgOptionBlocks`" + `. Each block builder implements ` + "`slack.Block`" + `, exposes only the fields valid for that Slack object, and validates before slack-go sends the request. Use ` + "`NewMessage()`" + ` and ` + "`Build()`" + ` when you need a complete JSON payload for another client.

- [Blocks](/reference/go/blocks)
- [Elements](/reference/go/elements)
- [Composition Objects](/reference/go/objects)
- [Payloads And Views](/reference/go/payloads)
- [Components](/reference/go/components)
- [Core JSON Types](/reference/go/core)
- [Validation And Errors](/reference/go/errors)
`
}

func expression(set *token.FileSet, node any) string {
	var output bytes.Buffer
	must(printer.Fprint(&output, set, node))
	return strings.ReplaceAll(output.String(), "slackapi.", "slack.")
}

func docText(group *ast.CommentGroup) string {
	if group == nil {
		return ""
	}
	return strings.TrimSpace(group.Text())
}

func must(err error) {
	if err != nil {
		panic(err)
	}
}
