# Native Haskell sending

`src/Send.hs` is the actual example transport exercised by `python bin/http-check`.
It takes any checked `ToJSON` payload and an application-owned `http-client`
manager. The executable in `src/Main.hs` constructs messages, webhooks, response
URL replies, modal views and App Home views through the native Slackblocks API.
It runs only against the loopback endpoint supplied by the checker.

Use `postPayload manager SlackAPI endpoint (Just token) payload` for Slack Web API
calls and `postPayload manager Webhook webhookURL Nothing payload` for incoming
webhooks or response URLs. Create production managers with `tlsManagerSettings`.
Do not log credentials or webhook URLs. `views.open` and `views.publish` require
their API envelope (`trigger_id`/`user_id` and `view`) around a checked view.

The example requires `http-client >=0.7.17 && <0.8`,
`http-client-tls >=0.3.6.3 && <0.4` and `http-types >=0.12.4 && <0.13`.
The minimum compiler is the library's GHC 9.6.7. CI independently resolves and
executes exact-lower, locked and latest transport graphs, with latest also on
GHC 9.14.1. `python bin/http-check --profile lower` selects the minimum graph;
`locked` selects the recorded GHC 9.6.7 graph; the default is `latest`.

The loopback server asserts complete JSON payload equality, arbitrary-precision
metadata, Unicode, explicit false, content type and authentication boundaries.
HTTP 429/500, redirects, malformed success bodies, `ok:false` and webhook errors
must fail. There are no automatic retries, so an ambiguous response cannot cause
an unbounded replay. Applications can apply their own bounded retry/deduplication
policy. These tests exercise a real HTTP client, but do not claim live Slack or
TLS-server acceptance.

The current [slack-web 2.3.0.0 source](https://hackage.haskell.org/package/slack-web-2.3.0.0/src/src/Web/Slack/Chat.hs)
offers text-encoded blocks/attachments in `PostMsgReq`, but has no message
metadata field. Its [current dependency bounds](https://hackage.haskell.org/package/slack-web-2.3.0.0/slack-web.cabal)
also require `base <4.22` and `aeson <2.3`, excluding this library's current
GHC 9.14 lane. The older `slack-api` targets Slack's RTM API. Neither provides
an intact, current-range adapter for this package, so the supported sending path
uses native JSON over HTTP rather than a conversion that drops fields. This
does not change either package's own support policy.
