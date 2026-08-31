# slackblocks for Go

Build validated Slack Block Kit payloads with a fluent Go API and no runtime
dependencies.

```go
block, err := slackblocks.NewSectionBlock().
    Text("Hello, *world*!").
    Build()
```

The module path is `github.com/nicklambourne/slackblocks/go/v2`. This directory
is part of the coordinated slackblocks monorepo; full installation, usage, and
API documentation will be published with the completed Go implementation.
