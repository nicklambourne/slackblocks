module example.com/slackblocks-sdk-compatibility

go 1.25

require (
	github.com/nicklambourne/slackblocks/go/v2 v2.8.0
	github.com/slack-go/slack v0.23.1
)

require github.com/gorilla/websocket v1.5.3 // indirect

replace github.com/nicklambourne/slackblocks/go/v2 => ..
