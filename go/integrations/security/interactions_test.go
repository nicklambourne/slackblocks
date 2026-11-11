package security_test

import (
	"net/http"
	"net/http/httptest"
	"net/url"
	"strings"
	"testing"

	slack "github.com/slack-go/slack"
)

// GHSA-pj2x-fjjr-q84c: URL query parameters are outside the signed request body.
// This test belongs only to the latest SDK profile; callers still verify signatures.
func TestInteractionPayloadComesOnlyFromPostBody(t *testing.T) {
	payload := `{"type":"block_actions","trigger_id":"body-trigger"}`
	query := url.Values{"payload": {`{"type":"block_actions","trigger_id":"query-trigger"}`}}.Encode()
	for _, test := range []struct {
		name      string
		body      string
		wantError bool
	}{
		{"body payload wins", url.Values{"payload": {payload}}.Encode(), false},
		{"query-only payload is rejected", "command=%2Fstatus", true},
		{"empty body payload is rejected", "payload=", true},
		{"malformed body payload is rejected", "payload=%7B", true},
		{"malformed form is rejected", "payload=%zz", true},
	} {
		t.Run(test.name, func(t *testing.T) {
			request := httptest.NewRequest(http.MethodPost, "/interactive?"+query, strings.NewReader(test.body))
			request.Header.Set("Content-Type", "application/x-www-form-urlencoded")
			callback, err := slack.InteractionCallbackParse(request)
			if test.wantError {
				if err == nil {
					t.Fatal("invalid POST body accepted; unsigned query payload must not be used")
				}
				return
			}
			if err != nil || callback.TriggerID != "body-trigger" {
				t.Fatalf("valid POST payload: callback=%+v, error=%v", callback, err)
			}
		})
	}
}
