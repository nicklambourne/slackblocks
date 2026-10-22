package slackblocks_test

import (
	"encoding/json"
	"testing"

	sb "github.com/nicklambourne/slackblocks/go/v3"
)

func TestRichTextLinkNativeUnsafe(t *testing.T) {
	omitted, err := sb.NewRichTextLink().URL("https://example.com").Build()
	if err != nil {
		t.Fatal(err)
	}
	data, err := json.Marshal(omitted)
	if err != nil {
		t.Fatal(err)
	}
	var decoded map[string]interface{}
	if err := json.Unmarshal(data, &decoded); err != nil {
		t.Fatal(err)
	}
	if _, present := decoded["unsafe"]; present {
		t.Fatal("omitted unsafe was serialized")
	}
	for _, unsafe := range []bool{false, true} {
		link, err := sb.NewRichTextLink().URL("https://example.com").Unsafe(unsafe).Build()
		if err != nil {
			t.Fatal(err)
		}
		data, err := json.Marshal(link)
		if err != nil {
			t.Fatal(err)
		}
		var decoded map[string]interface{}
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatal(err)
		}
		if decoded["unsafe"] != unsafe {
			t.Fatalf("unsafe = %#v, want %v", decoded["unsafe"], unsafe)
		}
	}
}
