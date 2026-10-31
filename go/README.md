# slackblocks for Go

[![Go](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fproxy.golang.org%2Fgithub.com%2Fnicklambourne%2Fslackblocks%2Fgo%2Fv3%2F%40latest&query=%24.Version&label=Go&color=00ADD8&logo=go&logoColor=white)](https://pkg.go.dev/github.com/nicklambourne/slackblocks/go/v3)
[![Go CI](https://github.com/nicklambourne/slackblocks/actions/workflows/go.yml/badge.svg?branch=master)](https://github.com/nicklambourne/slackblocks/actions/workflows/go.yml)

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

Every `New…` constructor returns a dedicated chainable builder that exposes only
the fields valid for that Slack object. Strings are promoted to the correct
Slack text objects where the wire format requires them and nested builders are
materialised automatically. Call `Build()` explicitly when you want eager
validation or a complete JSON-compatible payload such as `NewMessage()` for a
non-slack-go client.

Builder methods are typed, so a value of the wrong kind fails to compile:

- Nested fields take a role interface: `Accessory(Element)`, `Elements(...ContextElement)`,
  `Element(InputElement)`, `Chart(Chart)`, and `Rows(...[]TableCell)`. Only slackblocks
  builders implement these interfaces.
- Text fields take a string, which slackblocks sends as the field's text object type. Use the
  companion method, such as `TextObject(NewPlainText().Text("Hi").Emoji(true))`, to pass a
  text object.
- Closed value sets are named string types with constants, such as `ButtonStylePrimary`.
  Untyped string constants still work: `Style("primary")`. Supplied empty or
  unknown enum strings return `TypeMismatch` at build time. Leave an optional
  setter uncalled to preserve its omission or documented constructor default.
- Rich text styles take `RichTextStyle{"bold": true}`.

Passing a typed nil nested builder (for example, `Confirm(nil)`) returns a
`ValidationError` with category `TypeMismatch` and the field path when the parent
is built or marshalled. Omit an optional setter to omit that field; nil does not
mean omission. `MustBuild` still panics on validation errors by design.

`Set(field, value)` remains the raw wire-format escape hatch for fields slackblocks does not
model yet.

The module path is `github.com/nicklambourne/slackblocks/go/v3`:

```bash
go get github.com/nicklambourne/slackblocks/go/v3
```

Go 1.22+ is supported with the default slack-go v0.17.3 dependency. Applications
on Go 1.25+ can select the separately tested v0.23.1 SDK. The next release adds
a tested v0.31.0 profile for Go 1.26+, retaining the v0.30.1 file-URL security fix. See the
[Go support matrix](https://nicklambourne.github.io/slackblocks/usage/compatibility?language=go)
for selection commands, upstream requirements and update policy. The separate
`integrations` module tests the newer SDK; its alternate `go125.mod` preserves
the Go 1.25 profile. Updating slackblocks alone does not select the newer SDK
or backport its fixes to the older profiles.

This directory is part of the coordinated slackblocks monorepo. The Go module
implements the same shared JSON fixtures and validation categories as the
other language packages.

For explicit response visibility, replacement and delete-only payloads, see the
[response URL recipes](https://nicklambourne.github.io/slackblocks/usage/sending_messages?language=go).
`WebhookMessage` supports native `delete_original`; `MessageResponse` does not expose that field.

For exact numeric data-table cells, use `NewRawNumber().ValueNumber(json.Number("9007199254740993")).Text("9,007,199,254,740,993")`
with `encoding/json`. It preserves the JSON number without converting through
`float64`; malformed number syntax fails at `Build`. The existing `Value(float64)`
setter keeps its floating-point behavior. The separate `Text` field controls the
displayed label, not the numeric value used for sorting.

Builders retain nested builder references until `Build`, which produces an
independent wire snapshot. See [ownership and edits](https://nicklambourne.github.io/slackblocks/usage/using_blocks?language=go#ownership-and-edits)
for reusing builders, omitting optional fields and composing native helpers.
