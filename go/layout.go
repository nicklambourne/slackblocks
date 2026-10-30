package slackblocks

import "fmt"

var layoutChildTypes = stringSet("actions", "alert", "card", "carousel", "container", "context", "data_table", "data_visualization", "divider", "file", "header", "image", "input", "rich_text", "section", "table", "video")
var layoutFallbackTypes = stringSet("context", "divider", "header", "image", "rich_text", "section", "table", "video")

func layoutKeys(value Object, path string, keys ...string) error {
	allowed := stringSet(keys...)
	for key := range value {
		if !allowed[key] {
			return validationError(TypeMismatch, child(path, key), "unsupported property")
		}
	}
	return nil
}
func layoutRequired(value Object, field, path string) error {
	if _, ok := value[field]; !ok {
		return validationError(MissingRequired, child(path, field), "required")
	}
	return nil
}
func validateLayoutItem(value Object, path string) error {
	if err := layoutKeys(value, path, "block", "width", "horizontal_alignment"); err != nil {
		return err
	}
	if err := layoutRequired(value, "block", path); err != nil {
		return err
	}
	block, err := objectAt(value["block"], child(path, "block"))
	if err != nil {
		return err
	}
	if !layoutChildTypes[objectType(block)] {
		return validationError(TypeMismatch, child(path, "block"), "unsupported layout child")
	}
	if err := validateEnumField(value, "width", path, "narrow", "standard", "wide"); err != nil {
		return err
	}
	return validateEnumField(value, "horizontal_alignment", path, "start", "center", "end")
}
func validateLayoutRow(value Object, path string) error {
	if err := layoutKeys(value, path, "items"); err != nil {
		return err
	}
	if err := layoutRequired(value, "items", path); err != nil {
		return err
	}
	items, err := sliceAt(value["items"], child(path, "items"))
	if err != nil {
		return err
	}
	if err := sliceLength(items, child(path, "items"), limitLayoutItemsMinItems, limitLayoutItemsMaxItems); err != nil {
		return err
	}
	total, chart, wide := 0, 0, 0
	for i, raw := range items {
		p := fmt.Sprintf("%s.items[%d]", path, i)
		item, err := objectAt(raw, p)
		if err != nil {
			return err
		}
		if err := validateLayoutItem(item, p); err != nil {
			return err
		}
		weight := 2
		if item["width"] == "narrow" {
			weight = 1
		}
		if item["width"] == "wide" {
			weight = 3
		}
		total += weight
		block, _ := objectAt(item["block"], p)
		switch objectType(block) {
		case "carousel", "data_table":
			wide++
		case "data_visualization":
			wide++
			chart = weight
		}
	}
	if wide > 1 {
		return validationError(InvalidUsage, path, "at most one carousel, data table, or data visualization per row")
	}
	if chart > 0 && chart*2 <= total {
		return validationError(InvalidUsage, path, "a data visualization requires more than half the row weight")
	}
	return nil
}
func validateLayout(value Object, path string) error {
	for _, field := range []string{"label", "rows"} {
		if err := layoutRequired(value, field, path); err != nil {
			return err
		}
	}
	label, ok := value["label"].(string)
	if !ok {
		return validationError(TypeMismatch, child(path, "label"), "expected a string")
	}
	if err := stringLength(label, child(path, "label"), limitLayoutLabelMinLength, limitLayoutLabelMaxLength); err != nil {
		return err
	}
	rows, err := sliceAt(value["rows"], child(path, "rows"))
	if err != nil {
		return err
	}
	if err := sliceLength(rows, child(path, "rows"), limitLayoutRowsMinItems, limitLayoutRowsMaxItems); err != nil {
		return err
	}
	for i, raw := range rows {
		p := fmt.Sprintf("%s.rows[%d]", path, i)
		row, err := objectAt(raw, p)
		if err != nil {
			return err
		}
		if err := validateLayoutRow(row, p); err != nil {
			return err
		}
	}
	if raw, exists := value["is_label_visible"]; exists {
		if _, ok := raw.(bool); !ok {
			return validationError(TypeMismatch, child(path, "is_label_visible"), "expected a boolean")
		}
	}
	if err := validateEnumField(value, "grid_lines", path, "none", "rows", "outer"); err != nil {
		return err
	}
	if raw, exists := value["fallback"]; exists {
		blocks, err := sliceAt(raw, child(path, "fallback"))
		if err != nil {
			return err
		}
		for i, raw := range blocks {
			p := fmt.Sprintf("%s.fallback[%d]", path, i)
			block, err := objectAt(raw, p)
			if err != nil {
				return err
			}
			if !layoutFallbackTypes[objectType(block)] {
				return validationError(TypeMismatch, p, "unsupported layout fallback block")
			}
		}
	}
	return validateCompositeDepth(value, path, 0, false)
}

// compositeChildren follows modeled block positions only, never arbitrary metadata.
func compositeChildren(value Object) []any {
	if objectType(value) == "container" {
		v, _ := value["child_blocks"].([]any)
		return v
	}
	if objectType(value) != "layout" {
		return nil
	}
	var children []any
	rows, _ := value["rows"].([]any)
	for _, raw := range rows {
		row, ok := raw.(Object)
		if !ok {
			continue
		}
		items, _ := row["items"].([]any)
		for _, raw := range items {
			if item, ok := raw.(Object); ok {
				children = append(children, item["block"])
			}
		}
	}
	return children
}
func compositeSurfaceChildren(value Object) []any {
	children := compositeChildren(value)
	if objectType(value) == "layout" {
		fallback, _ := value["fallback"].([]any)
		children = append(children, fallback...)
	}
	return children
}
func validateCompositeDepth(value Object, path string, depth int, hasLayout bool) error {
	kind := objectType(value)
	hasLayout = hasLayout || kind == "layout"
	if kind == "container" || kind == "layout" {
		depth++
	}
	if hasLayout && depth > 2 {
		return validationError(InvalidUsage, path, "layout/container nesting exceeds two composite levels")
	}
	for _, raw := range compositeChildren(value) {
		if block, ok := raw.(Object); ok {
			if err := validateCompositeDepth(block, path, depth, hasLayout); err != nil {
				return err
			}
		}
	}
	return nil
}

func containsInput(block Object) bool {
	if objectType(block) == "input" {
		return true
	}
	for _, raw := range compositeSurfaceChildren(block) {
		if b, ok := raw.(Object); ok && containsInput(b) {
			return true
		}
	}
	return false
}

func containsLayout(block Object) bool {
	if objectType(block) == "layout" {
		return true
	}
	for _, raw := range compositeChildren(block) {
		if b, ok := raw.(Object); ok && containsLayout(b) {
			return true
		}
	}
	return false
}
func blockCost(block Object, nested bool) int {
	kind := objectType(block)
	children := compositeChildren(block)
	if kind == "layout" || (kind == "container" && (nested || containsLayout(block))) {
		total := 1
		for _, raw := range children {
			if b, ok := raw.(Object); ok {
				total += blockCost(b, nested || kind == "layout")
			}
		}
		if kind == "layout" {
			fallback := 0
			if items, ok := block["fallback"].([]any); ok {
				for _, raw := range items {
					if b, ok := raw.(Object); ok {
						fallback += blockCost(b, false)
					}
				}
			}
			if fallback > total {
				return fallback
			}
		}
		return total
	}
	return 1
}
