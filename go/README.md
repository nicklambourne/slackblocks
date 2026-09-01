# slackblocks for Go

Build validated Slack Block Kit payloads with a fluent Go API that integrates
directly with [`slack-go/slack`](https://github.com/slack-go/slack).

```go
func send(ctx context.Context, client *slack.Client) (string, error) {
    _, timestamp, err := client.PostMessageContext(
        ctx,
        "C123456",
        slack.MsgOptionText("Build #482 passed", false),
        slack.MsgOptionBlocks(
            slackblocks.NewHeaderBlock().Text("Build #482 passed :white_check_mark:"),
            slackblocks.NewSectionBlock().Fields(
                "*Branch*\n`main`",
                "*Tests*\n1,247 passed",
            ),
            slackblocks.NewActionsBlock().Elements(
                slackblocks.NewButton().
                    Text("View build").
                    ActionID("view").
                    URL("https://ci.example.com/482"),
            ),
        ),
    )
    return timestamp, err
}
```

Every top-level block builder implements `slack.Block`, so channels and message
options remain ordinary slack-go values. slack-go marshals each builder before
sending; if slackblocks validation fails, the call returns a typed
`ValidationError` without making the HTTP request.

Every `New…` constructor returns a chainable builder. Strings are promoted to
the correct Slack text objects where the wire format requires them and nested
builders are materialised automatically. Call `Build()` explicitly when you
want eager validation or a complete JSON-compatible payload such as
`NewMessage()` for a non-slack-go client.

The module path is `github.com/nicklambourne/slackblocks/go/v2`:

```bash
go get github.com/nicklambourne/slackblocks/go/v2
```

This directory is part of the coordinated slackblocks monorepo. The Go module
implements the same shared JSON fixtures and validation categories as the
Python and TypeScript packages.
