package slackblocks

import (
	"reflect"
	"strings"
	"testing"
	"unicode"
)

func TestEveryNamedFieldMethodWritesItsExpectedWireKey(t *testing.T) {
	t.Parallel()

	overrides := map[string]string{
		"Expanded":         "_expanded",
		"SlackFileID":      "id",
		"SlackFileURL":     "url",
		"UserGroupID":      "usergroup_id",
		"VisibleToUserIDs": "visible_to_user_ids",
	}
	excluded := map[string]bool{
		"Build":       true,
		"MarshalJSON": true,
		"MustBuild":   true,
		"Set":         true,
	}

	builderType := reflect.TypeOf((*builder)(nil))
	for index := 0; index < builderType.NumMethod(); index++ {
		method := builderType.Method(index)
		if excluded[method.Name] {
			continue
		}
		t.Run(method.Name, func(t *testing.T) {
			core := newBuilder("test", "")
			bound := reflect.ValueOf(core).MethodByName(method.Name)
			if bound.Type().NumIn() != 1 {
				t.Fatalf("named field method has %d parameters, want 1", bound.Type().NumIn())
			}
			argumentType := bound.Type().In(0)
			if bound.Type().IsVariadic() {
				argument := reflect.MakeSlice(argumentType, 1, 1)
				argument.Index(0).Set(sampleFieldValue(argumentType.Elem()))
				bound.CallSlice([]reflect.Value{argument})
			} else {
				bound.Call([]reflect.Value{sampleFieldValue(argumentType)})
			}

			wireKey := overrides[method.Name]
			if wireKey == "" {
				wireKey = snakeCaseMethod(method.Name)
			}
			if _, ok := core.values[wireKey]; !ok {
				t.Fatalf("%s did not write expected wire key %q; got %#v", method.Name, wireKey, core.values)
			}
		})
	}
}

func sampleFieldValue(valueType reflect.Type) reflect.Value {
	switch valueType.Kind() {
	case reflect.Interface:
		return reflect.ValueOf("value")
	case reflect.String:
		return reflect.ValueOf("value").Convert(valueType)
	case reflect.Bool:
		return reflect.ValueOf(true).Convert(valueType)
	case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64:
		return reflect.ValueOf(int64(1)).Convert(valueType)
	case reflect.Float32, reflect.Float64:
		return reflect.ValueOf(float64(1)).Convert(valueType)
	default:
		panic("unsupported named field parameter: " + valueType.String())
	}
}

func snakeCaseMethod(name string) string {
	runes := []rune(name)
	var result strings.Builder
	for index, current := range runes {
		if index > 0 && unicode.IsUpper(current) {
			previous := runes[index-1]
			nextIsLower := index+1 < len(runes) && unicode.IsLower(runes[index+1])
			if unicode.IsLower(previous) || unicode.IsDigit(previous) || nextIsLower {
				result.WriteByte('_')
			}
		}
		result.WriteRune(unicode.ToLower(current))
	}
	return result.String()
}
