package slackblocks

import (
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"testing"
)

func TestValidationTablesMatchSharedVocabulary(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "spec", "vocabulary.json"))
	if err != nil {
		t.Fatal(err)
	}
	var vocabulary struct {
		SlackIconNames    []string            `json:"slack_icon_names"`
		SurfaceBlockTypes map[string][]string `json:"surface_block_types"`
	}
	if err := json.Unmarshal(data, &vocabulary); err != nil {
		t.Fatal(err)
	}
	if got, want := sortedKeys(slackIconNames), sortedCopy(vocabulary.SlackIconNames); !reflect.DeepEqual(got, want) {
		t.Fatalf("slack icon names differ from spec/vocabulary.json\nwant: %v\n got: %v", want, got)
	}
	if len(surfaceBlocks) != len(vocabulary.SurfaceBlockTypes) {
		t.Fatalf("surfaces differ from spec/vocabulary.json: %d vs %d", len(surfaceBlocks), len(vocabulary.SurfaceBlockTypes))
	}
	for surface, types := range vocabulary.SurfaceBlockTypes {
		if got, want := sortedKeys(surfaceBlocks[surface]), sortedCopy(types); !reflect.DeepEqual(got, want) {
			t.Fatalf("%s block types differ from spec/vocabulary.json\nwant: %v\n got: %v", surface, want, got)
		}
	}
}

func sortedKeys(set map[string]bool) []string {
	keys := make([]string, 0, len(set))
	for key := range set {
		keys = append(keys, key)
	}
	sort.Strings(keys)
	return keys
}

func sortedCopy(values []string) []string {
	copied := append([]string(nil), values...)
	sort.Strings(copied)
	return copied
}
