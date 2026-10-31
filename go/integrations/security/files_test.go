package security_test

import (
	"context"
	"io"
	"net/http"
	"strings"
	"testing"

	slack "github.com/slack-go/slack"
)

type httpRecorder func(*http.Request) (*http.Response, error)

func (record httpRecorder) Do(request *http.Request) (*http.Response, error) {
	return record(request)
}

// This suite belongs to the v0.30.1 profile: older supported SDKs do not
// include this security fix. No request reaches a network transport.
func TestFileURLsProtectTheToken(t *testing.T) {
	operations := map[string]func(*slack.Client, string) error{
		"download": func(client *slack.Client, address string) error {
			return client.GetFile(address, io.Discard)
		},
		"download with context": func(client *slack.Client, address string) error {
			return client.GetFileContext(context.Background(), address, io.Discard)
		},
		"upload": func(client *slack.Client, address string) error {
			return client.UploadToURL(context.Background(), slack.UploadToURLParameters{
				UploadURL: address, Filename: "status.txt", Content: "healthy",
			})
		},
	}
	for name, operation := range operations {
		for _, destination := range []struct {
			address string
			allowed bool
		}{
			{"https://untrusted.example/status.txt", false},
			{"https://files.slack.com.untrusted.example/status.txt", false},
			{"https://files.slack.com@untrusted.example/status.txt", false},
			{"http://files.slack.com/status.txt", false},
			{"https://files.slack.com/status.txt", true},
			{"https://files.slack-gov.com/status.txt", true},
			{"https://test-api.example/status.txt", true},
		} {
			t.Run(name+"/"+destination.address, func(t *testing.T) {
				requests := 0
				recorder := httpRecorder(func(request *http.Request) (*http.Response, error) {
					requests++
					if request.Header.Get("Authorization") != "Bearer test-only-token" {
						t.Error("allowed SDK request lost its authorization")
					}
					if request.Body != nil {
						_, _ = io.Copy(io.Discard, request.Body)
						_ = request.Body.Close()
					}
					return &http.Response{
						StatusCode: http.StatusOK, Request: request,
						Header: http.Header{"Content-Type": {"application/octet-stream"}},
						Body:   io.NopCloser(strings.NewReader("healthy")),
					}, nil
				})
				client := slack.New("test-only-token", slack.OptionHTTPClient(recorder),
					slack.OptionAPIURL("https://test-api.example/"))
				err := operation(client, destination.address)
				if destination.allowed {
					if err != nil || requests != 1 {
						t.Fatalf("allowed file URL: error=%v, requests=%d; want one successful request", err, requests)
					}
				} else if err == nil || requests != 0 {
					t.Fatalf("untrusted file URL: error=%v, requests=%d; want rejection before HTTP", err, requests)
				}
			})
		}
	}
}
