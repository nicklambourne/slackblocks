module example.com/slackblocks-multiselect

go 1.25

require github.com/nicklambourne/slackblocks/go/v2 v2.8.0

require (
	github.com/gorilla/websocket v1.5.3 // indirect
	github.com/slack-go/slack v0.23.1 // indirect
)

replace github.com/nicklambourne/slackblocks/go/v2 => ../../../../go
