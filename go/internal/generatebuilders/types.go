package main

import (
	"fmt"
	"sort"
	"strings"
)

// marker describes one generated role interface. Its unexported method restricts
// implementations to this package, so only slackblocks builders can satisfy it.
type marker struct {
	name        string
	method      string
	embeds      string
	description string
}

// markers maps shared model role names to their Go interfaces. The model's "Text" role is
// TextObject in Go, where Text is already the name of the fluent method.
var markers = map[string]marker{
	"Block":                  {name: "Block", method: "slackblocksBlock", description: "Block is a layout block, or a component that expands to layout blocks, accepted by message, modal, App Home, attachment, and container block collections."},
	"Element":                {name: "Element", method: "slackblocksElement", description: "Element is an interactive, visual, or input element that can appear in an actions block or as a section accessory."},
	"InputElement":           {name: "InputElement", method: "slackblocksInputElement", embeds: "Element", description: "InputElement is an element that can be placed in an input block."},
	"ContextElement":         {name: "ContextElement", method: "slackblocksContextElement", description: "ContextElement is an image element or text object that can appear in a context block."},
	"ContextActionsElement":  {name: "ContextActionsElement", method: "slackblocksContextActionsElement", description: "ContextActionsElement is feedback buttons or an icon button that can appear in a context actions block."},
	"TableCell":              {name: "TableCell", method: "slackblocksTableCell", description: "TableCell is raw text or a rich text block that can appear as a table block cell."},
	"DataTableCell":          {name: "DataTableCell", method: "slackblocksDataTableCell", description: "DataTableCell is raw text, a raw number, or a rich text block that can appear as a data table cell."},
	"Chart":                  {name: "Chart", method: "slackblocksChart", description: "Chart is a pie, bar, area, or line chart that a data visualization block can display."},
	"RichTextBlockElement":   {name: "RichTextBlockElement", method: "slackblocksRichTextBlockElement", description: "RichTextBlockElement is a section, list, preformatted block, or quote inside a rich text block."},
	"RichTextSectionElement": {name: "RichTextSectionElement", method: "slackblocksRichTextSectionElement", description: "RichTextSectionElement is inline rich text: text, a link, an emoji, or a channel, user, or user group mention."},
	"Text":                   {name: "TextObject", method: "slackblocksTextObject", description: "TextObject is a plain_text or mrkdwn text composition object."},
}

// componentBlocks are hand-written or model-less builders that also satisfy Block.
var componentBlocks = []string{"AccordionBuilder", "PaginatorBuilder", "AccordionSectionBuilder"}

// accordionSectionFields types the accordion section, which is a Go component and not part of
// the shared model.
var accordionSectionFields = map[string]modelField{
	"Title":    {GoMethod: "Title", Wire: "title", Kind: "text", Type: "PlainText", Coerce: "plain_text", Description: "Sets the section heading.", Required: true},
	"Subtitle": {GoMethod: "Subtitle", Wire: "subtitle", Kind: "text", Type: "Text", Coerce: "mrkdwn", Description: "Sets supporting text shown below the heading."},
	"Blocks":   {GoMethod: "Blocks", Wire: "blocks", Kind: "list", Type: "Block", Description: "Adds the blocks revealed when the section is expanded."},
	"Expanded": {GoMethod: "Expanded", Wire: "_expanded", Kind: "boolean", Description: "Sets whether the section starts expanded."},
	"BlockID":  {GoMethod: "BlockID", Wire: "block_id", Kind: "string", Description: "Sets a unique identifier for this block."},
}

// typing resolves shared model types to Go parameter types.
type typing struct {
	builders map[string]string // model type name -> Go builder type
	enums    []modelEnum
}

func newTyping(shared model) typing {
	builders := map[string]string{}
	for _, item := range shared.Types {
		builders[item.Name] = strings.TrimPrefix(item.GoConstructor, "New") + "Builder"
	}
	return typing{builders: builders, enums: shared.Enums}
}

// goType returns the Go type for a model object or list item type.
func (t typing) goType(name string) string {
	if role, ok := markers[name]; ok {
		return role.name
	}
	if builder, ok := t.builders[name]; ok {
		return "*" + builder
	}
	panic(fmt.Sprintf("model type %s has no Go builder", name))
}

// objectMethodName is the companion method that accepts a text object for a text field.
func objectMethodName(method, kind string) string {
	if kind == "textList" {
		return strings.TrimSuffix(method, "s") + "Objects"
	}
	return method + "Object"
}

// typedMethods renders the fluent methods for one model field on one concrete builder.
func (t typing) typedMethods(typeName string, field modelField, comment string) string {
	method := field.GoMethod
	var output strings.Builder
	write := func(doc, name, params, body string) {
		output.WriteString(doc)
		fmt.Fprintf(&output, "func (b *%s) %s(%s) *%s {\n%s\treturn b\n}\n\n", typeName, name, params, typeName, body)
	}
	call := func(arguments string) string { return fmt.Sprintf("\tb.core.%s(%s)\n", method, arguments) }
	spread := func(values string) string {
		return fmt.Sprintf("\titems := make([]any, len(%s))\n\tfor index, value := range %s {\n\t\titems[index] = value\n\t}\n\tb.core.%s(items...)\n", values, values, method)
	}
	scalars := map[string]string{"string": "string", "boolean": "bool", "int": "int", "long": "int64", "double": "float64", "number": "float64"}

	switch field.Kind {
	case "string", "boolean", "int", "long", "double", "number":
		write(comment, method, "value "+scalars[field.Kind], call("value"))
	case "stringList":
		write(comment, method, "values ...string", call("values..."))
	case "enum":
		write(comment, method, "value "+field.Type, call("string(value)"))
	case "map":
		write(comment, method, "value Object", call("value"))
	case "style":
		write(comment, method, "value RichTextStyle", call("value.object()"))
	case "object":
		write(comment, method, "value "+t.goType(field.Type), call("value"))
	case "list":
		write(comment, method, "values ..."+t.goType(field.Type), spread("values"))
	case "rows":
		body := fmt.Sprintf("\tfor _, row := range rows {\n\t\tcells := make([]any, len(row))\n\t\tfor index, cell := range row {\n\t\t\tcells[index] = cell\n\t\t}\n\t\tb.core.%s(cells)\n\t}\n", method)
		write(comment, method, "rows ...[]"+t.goType(field.Type), body)
	case "text":
		write(comment, method, "value string", call("value"))
		objectDoc := wrapComment(fmt.Sprintf("%s is %s for a text object, such as NewPlainText().Text(\"...\").Emoji(true).", objectMethodName(method, "text"), method))
		write(objectDoc, objectMethodName(method, "text"), "value "+t.goType(field.Type), call("value"))
	case "textList":
		write(comment, method, "values ...string", fmt.Sprintf("\tfor _, value := range values {\n\t\tb.core.%s(value)\n\t}\n", method))
		objectDoc := wrapComment(fmt.Sprintf("%s is %s for text objects. Each call appends to any values already added.", objectMethodName(method, "textList"), method))
		write(objectDoc, objectMethodName(method, "textList"), "values ..."+t.goType(field.Type), spread("values"))
	default:
		panic(fmt.Sprintf("unsupported field kind %s for %s.%s", field.Kind, typeName, method))
	}
	return output.String()
}

// markerDeclarations renders the role interfaces, their implementations, the enums, and the
// rich text style type.
func (t typing) markerDeclarations(shared model) string {
	implementers := map[string][]string{}
	add := func(role, builder string) { implementers[role] = append(implementers[role], builder) }
	for _, item := range shared.Types {
		builder := strings.TrimPrefix(item.GoConstructor, "New") + "Builder"
		roles := append([]string(nil), item.Implements...)
		switch item.Package {
		case "block":
			roles = append(roles, "Block")
		case "element":
			roles = append(roles, "Element")
		}
		for _, role := range roles {
			add(role, builder)
			if role == "InputElement" {
				add("Element", builder)
			}
		}
	}
	for _, builder := range componentBlocks {
		add("Block", builder)
	}

	var output strings.Builder
	for _, key := range sortedMarkerKeys() {
		role := markers[key]
		builders := uniqueSorted(implementers[key])
		output.WriteString(wrapComment(role.description))
		output.WriteString(wrapComment("Implemented by " + joinNames(pointers(builders)) + "."))
		if role.embeds != "" {
			fmt.Fprintf(&output, "type %s interface {\n\t%s\n\t%s()\n}\n\n", role.name, role.embeds, role.method)
		} else {
			fmt.Fprintf(&output, "type %s interface {\n\t%s()\n}\n\n", role.name, role.method)
		}
		for _, builder := range builders {
			fmt.Fprintf(&output, "func (*%s) %s() {}\n", builder, role.method)
		}
		output.WriteString("\n")
	}

	for _, enum := range t.enums {
		output.WriteString(wrapComment(enum.Name + " is a named Slack value. " + sentence(enum.Description) + " Untyped string constants such as \"" + enum.Constants[0].Wire + "\" also convert to it."))
		fmt.Fprintf(&output, "type %s string\n\nconst (\n", enum.Name)
		for _, constant := range enum.Constants {
			output.WriteString(indentComment(wrapComment(constant.Description)))
			fmt.Fprintf(&output, "\t%s%s %s = %q\n", enum.Name, camelConstant(constant.Name), enum.Name, constant.Wire)
		}
		output.WriteString(")\n\n")
	}

	output.WriteString(`// RichTextStyle sets formatting flags on inline rich text, such as
// RichTextStyle{"bold": true, "italic": false}. Text and links support bold, italic, strike,
// and code; mentions support bold, italic, strike, highlight, client_highlight, and unlink.
type RichTextStyle map[string]bool

func (style RichTextStyle) object() Object {
	object := Object{}
	for flag, enabled := range style {
		object[flag] = enabled
	}
	return object
}

`)
	return output.String()
}

func sortedMarkerKeys() []string {
	keys := make([]string, 0, len(markers))
	for key := range markers {
		keys = append(keys, key)
	}
	sort.Slice(keys, func(left, right int) bool { return markers[keys[left]].name < markers[keys[right]].name })
	return keys
}

func uniqueSorted(values []string) []string {
	seen := map[string]bool{}
	result := []string{}
	for _, value := range values {
		if !seen[value] {
			seen[value] = true
			result = append(result, value)
		}
	}
	sort.Strings(result)
	return result
}

func pointers(values []string) []string {
	result := make([]string, len(values))
	for index, value := range values {
		result[index] = "*" + value
	}
	return result
}

func camelConstant(name string) string {
	parts := strings.Split(strings.ToLower(name), "_")
	for index, part := range parts {
		parts[index] = strings.ToUpper(part[:1]) + part[1:]
	}
	return strings.Join(parts, "")
}

func indentComment(comment string) string {
	lines := strings.Split(strings.TrimSuffix(comment, "\n"), "\n")
	for index, line := range lines {
		lines[index] = "\t" + line
	}
	return strings.Join(lines, "\n") + "\n"
}
