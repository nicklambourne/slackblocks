# Elixir sending examples

This separate Mix project demonstrates transport choices and tests them against a
local HTTP server. Only Jason and Decimal are runtime dependencies of slackblocks.
The sending examples are application code, not part of the published core API.

## Community client and direct HTTP

The community `slack_elixir` package supplies `Slack.API.client(token)`, which
returns a Req request with Slack's API URL and bearer authentication. In the
published 1.2.2 package, `Slack.API.post/3` sends `form: args`; use the configured
request with Req's `json:` option for complete nested Block Kit payloads.
The snippets assume `token`, a native `message`, and `base_url` set to
`"https://slack.com/api"` in your application; tests supply a localhost URL.

```elixir
client = Slack.API.client(token) |> Req.merge(base_url: base_url)
Req.post(client,
  url: "chat.postMessage",
  json: Slackblocks.to_map!(message),
  retry: false
)
```

Using Req directly is equally supported:

```elixir
Req.post(base_url <> "/chat.postMessage",
  auth: {:bearer, token},
  json: Slackblocks.to_map!(message),
  retry: false
)
```

Check both the HTTP status and Slack's `ok` field. The executable examples in
`lib/sending.ex` handle HTTP failures, Slack errors and malformed responses.
They retry only explicit 429 responses: at most three requests, with two seconds
maximum for each Retry-After delay. Missing, malformed or longer delays return
an error rather than retrying before Slack permits it. Req's automatic retries
and redirects are disabled. Timeouts are not retried because the original POST
may already have been accepted. Production applications should choose their own
policy and handle delays outside an interaction's acknowledgement deadline.
Transport exceptions are returned unchanged: their concrete module depends on
the selected Req/Finch version; callers may inspect their `reason` field.

## Incoming webhooks and Phoenix

Use `Slackblocks.WebhookMessage` with an incoming webhook URL. Webhook success is
plain text `ok`, unlike Web API JSON. The `webhook/3` example checks that contract
and sends no bearer authentication header.

`Sending.response_payload/0` produces an ordinary validated map, suitable as the
body of a Phoenix JSON response. It uses no Phoenix-specific API. Signature
verification, quick acknowledgement, routing and any later asynchronous response
remain application responsibilities; the core library does not implement a bot
framework or web server.

## Reproducible checks

From this directory:

```sh
mix deps.get
mix test --warnings-as-errors
```

From the package directory, test against the actual unpacked Hex artifact:

```sh
python3 tools/integration_check.py --profile locked
python3 tools/integration_check.py --profile lower
python3 tools/integration_check.py --profile latest
```

- Locked uses this project's committed lock.
- Lower explicitly pins slack_elixir 1.2.2, Req 0.5.0, Ecto 3.14.0, Jason 1.4.5
  and Decimal 3.0.0. Transitive dependencies resolve normally; this is not a claim
  to exercise every historical transitive version.
- Latest resolves fresh within the declared ranges without the lock.

All profiles compile and test the extracted package, send actual request bytes to
localhost, and compile a production consumer. They verify bearer headers,
content type, Unicode, empty objects/lists, null/false/zero, metadata, attachments,
plans and data tables. Ecto co-installation and checked decimal parsing are tested.
The locked profile first compiles a Jason-only consumer, then adds slackblocks
without cleaning its build and verifies Decimal decoding and consolidated
protocols. No real Slack workspace, token or webhook is used in these tests.

The package's complete local checker runs all three profiles. HTTP test state is
per-test; no global Req defaults or application environment is changed.

Sources: [published client release](https://hex.pm/packages/slack_elixir/1.2.2),
[client source archive](https://repo.hex.pm/tarballs/slack_elixir-1.2.2.tar),
[Req request options](https://req.hexdocs.pm/Req.html#new/1),
[Ecto 3.14 requirements](https://hex.pm/api/packages/ecto/releases/3.14.0).
