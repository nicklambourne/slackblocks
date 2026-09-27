package main

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// The shared Block Kit model describes every builder's fields, their limits, required fields,
// validation rules, and Slack documentation links. The Java and C# generators read it too, and it is
// cross-checked against builder_methods.json by java/generator/generate_models.py --check-go.
const modelFile = "spec/model.json"

// The shared limits registry supplies the documented Slack limits referenced by the model.
const limitsFile = "spec/limits.json"

type model struct {
	Enums []modelEnum `json:"enums"`
	Types []modelType `json:"types"`
}

type modelEnum struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	Constants   []struct {
		Name        string `json:"name"`
		Wire        string `json:"wire"`
		Description string `json:"description"`
	} `json:"constants"`
}

type modelType struct {
	Name          string       `json:"name"`
	Package       string       `json:"package"`
	Implements    []string     `json:"implements"`
	GoConstructor string       `json:"goConstructor"`
	Description   string       `json:"description"`
	DocURL        string       `json:"docUrl"`
	Rules         []string     `json:"rules"`
	Fields        []modelField `json:"fields"`
}

type modelField struct {
	GoMethod    string   `json:"goMethod"`
	Wire        string   `json:"wire"`
	Kind        string   `json:"kind"`
	Type        string   `json:"type"`
	Coerce      string   `json:"coerce"`
	Flags       []string `json:"flags"`
	Description string   `json:"description"`
	Required    bool     `json:"required"`
	Limits      string   `json:"limits"`
}

// builderDocs holds the documentation generated for one constructor's builder.
type builderDocs struct {
	typeComment string
	methods     map[string]string
	fields      map[string]modelField
}

// loadDocs reads the shared model and returns generated documentation keyed by Go constructor,
// together with the model itself for type generation.
func loadDocs(root string) (map[string]builderDocs, model) {
	repository := filepath.Join(root, "..")
	modelData, err := os.ReadFile(filepath.Join(repository, modelFile))
	must(err)
	var shared model
	must(json.Unmarshal(modelData, &shared))

	limitsData, err := os.ReadFile(filepath.Join(repository, limitsFile))
	must(err)
	var limits map[string]any
	must(json.Unmarshal(limitsData, &limits))

	enums := map[string][]string{}
	for _, enum := range shared.Enums {
		values := make([]string, 0, len(enum.Constants))
		for _, constant := range enum.Constants {
			values = append(values, constant.Wire)
		}
		enums[enum.Name] = values
	}

	docs := map[string]builderDocs{}
	for _, item := range shared.Types {
		methods := map[string]string{}
		fields := map[string]modelField{}
		var required []string
		for _, field := range item.Fields {
			if field.GoMethod == "" {
				continue
			}
			if field.Required {
				required = append(required, field.GoMethod)
			}
			methods[field.GoMethod] = methodComment(field, limits, enums)
			fields[field.GoMethod] = field
		}
		docs[item.GoConstructor] = builderDocs{
			typeComment: typeComment(item, required),
			methods:     methods,
			fields:      fields,
		}
	}
	accordion := builderDocs{
		typeComment: "// AccordionSectionBuilder is the concrete fluent builder returned by NewAccordionSection.\n//\n// One independently collapsible section of an accordion, rendered as a Slack container block.\n//\n//   - Required: Title and Blocks.\n",
		methods:     map[string]string{},
		fields:      accordionSectionFields,
	}
	for name, field := range accordionSectionFields {
		accordion.methods[name] = methodComment(field, limits, enums)
	}
	docs["NewAccordionSection"] = accordion
	return docs, shared
}

func typeComment(item modelType, required []string) string {
	typeName := strings.TrimPrefix(item.GoConstructor, "New") + "Builder"
	var output strings.Builder
	fmt.Fprintf(&output, "// %s is the concrete fluent builder returned by %s.\n", typeName, item.GoConstructor)
	fmt.Fprintf(&output, "//\n// %s\n", sentence(item.Description))
	rules := append([]string(nil), item.Rules...)
	if len(required) > 0 {
		rules = append([]string{"Required: " + joinNames(required) + "."}, rules...)
	}
	if len(rules) > 0 {
		output.WriteString("//\n")
		for _, rule := range rules {
			fmt.Fprintf(&output, "//   - %s\n", sentence(rule))
		}
	}
	if item.DocURL != "" {
		fmt.Fprintf(&output, "//\n// See %s for Slack's reference.\n", item.DocURL)
	}
	return output.String()
}

func methodComment(field modelField, limits map[string]any, enums map[string][]string) string {
	description := strings.TrimSpace(field.Description)
	for _, prefix := range []string{"Sets ", "Adds "} {
		if strings.HasPrefix(description, prefix) {
			description = strings.ToLower(prefix[:1]) + description[1:]
			break
		}
	}
	notes := []string{field.GoMethod + " " + sentence(description)}
	if field.Required {
		notes = append(notes, "Required.")
	}
	if field.Limits != "" {
		notes = append(notes, limitSentence(limits, field.Limits))
	}
	switch field.Kind {
	case "text":
		notes = append(notes, fmt.Sprintf("The string is sent as a %s text object; use %s to pass a text object instead.", field.Coerce, objectMethodName(field.GoMethod, "text")))
	case "textList":
		notes = append(notes, fmt.Sprintf("Strings are sent as %s text objects; use %s to pass text objects. Each call appends to any values already added.", field.Coerce, objectMethodName(field.GoMethod, "textList")))
	case "list", "stringList", "rows":
		notes = append(notes, "Each call appends to any values already added.")
	case "enum":
		notes = append(notes, "Accepted values: "+joinNames(quoteAll(enums[field.Type]))+".")
	case "style":
		notes = append(notes, "Pass an Object of boolean flags, such as `Object{\"bold\": true}`. Supported flags: "+joinNames(quoteAll(field.Flags))+".")
	case "map":
		notes = append(notes, "Pass an Object or map of JSON-compatible values.")
	}
	return wrapComment(strings.Join(notes, " "))
}

// limitSentence renders one spec/limits.json leaf as prose, matching the Java generator.
func limitSentence(limits map[string]any, path string) string {
	var node any = limits
	for _, part := range strings.Split(path, ".") {
		object, ok := node.(map[string]any)
		if !ok {
			panic(fmt.Sprintf("limits path %s does not exist", path))
		}
		node = object[part]
	}
	leaf, ok := node.(map[string]any)
	if !ok {
		panic(fmt.Sprintf("limits path %s is not a limit", path))
	}
	number := func(key string) (int, bool) {
		value, present := leaf[key].(float64)
		return int(value), present
	}
	if itemMax, present := number("item_max_length"); present {
		maxItems, _ := number("max_items")
		return fmt.Sprintf("Slack allows at most %d items, each at most %d characters.", maxItems, itemMax)
	}
	if low, hasLow := number("min_length"); hasLow || leaf["max_length"] != nil {
		high, hasHigh := number("max_length")
		switch {
		case hasLow && low > 0 && hasHigh:
			return fmt.Sprintf("Must be between %d and %d characters.", low, high)
		case hasHigh:
			return fmt.Sprintf("Slack allows at most %d characters.", high)
		default:
			return "Must not be empty."
		}
	}
	if low, hasLow := number("min_items"); hasLow || leaf["max_items"] != nil {
		high, hasHigh := number("max_items")
		switch {
		case hasLow && low > 0 && hasHigh:
			return fmt.Sprintf("Must contain between %d and %d items.", low, high)
		case hasHigh:
			return fmt.Sprintf("Slack allows at most %d items.", high)
		case low == 1:
			return "Must not be empty."
		default:
			return fmt.Sprintf("Must contain at least %d items.", low)
		}
	}
	if low, hasLow := number("min"); hasLow || leaf["max"] != nil {
		high, hasHigh := number("max")
		switch {
		case hasLow && hasHigh:
			return fmt.Sprintf("Must be between %d and %d.", low, high)
		case hasHigh:
			return fmt.Sprintf("Must be at most %d.", high)
		default:
			return fmt.Sprintf("Must be at least %d.", low)
		}
	}
	if minimum, present := number("exclusive_min"); present {
		return fmt.Sprintf("Must be greater than %d.", minimum)
	}
	panic(fmt.Sprintf("unsupported limit at %s", path))
}

func sentence(text string) string {
	text = strings.TrimSpace(text)
	if text == "" || strings.HasSuffix(text, ".") {
		return text
	}
	return text + "."
}

func quoteAll(values []string) []string {
	quoted := make([]string, 0, len(values))
	for _, value := range values {
		quoted = append(quoted, `"`+value+`"`)
	}
	return quoted
}

func joinNames(names []string) string {
	switch len(names) {
	case 0:
		return ""
	case 1:
		return names[0]
	case 2:
		return names[0] + " and " + names[1]
	default:
		return strings.Join(names[:len(names)-1], ", ") + ", and " + names[len(names)-1]
	}
}

// wrapComment renders prose as a Go doc comment wrapped near 100 columns.
func wrapComment(text string) string {
	const width = 96
	var output strings.Builder
	line := "//"
	for _, word := range strings.Fields(text) {
		if len(line)+1+len(word) > width && line != "//" {
			output.WriteString(line + "\n")
			line = "//"
		}
		line += " " + word
	}
	output.WriteString(line + "\n")
	return output.String()
}
