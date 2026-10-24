module example.com/slackblocks-sdk-compatibility

go 1.26

require (
	github.com/nicklambourne/slackblocks/go/v3 v3.0.0
	github.com/slack-go/slack v0.30.1
)

require github.com/gorilla/websocket v1.5.3 // indirect

replace github.com/nicklambourne/slackblocks/go/v3 => ..
