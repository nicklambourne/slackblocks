module example.com/slackblocks-multiselect

go 1.22

require github.com/nicklambourne/slackblocks/go/v2 v2.8.0

require (
	github.com/gorilla/websocket v1.5.3 // indirect
	github.com/slack-go/slack v0.17.3 // indirect
)

replace github.com/nicklambourne/slackblocks/go/v2 => ../../../../go
