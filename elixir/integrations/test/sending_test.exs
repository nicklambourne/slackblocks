defmodule Slackblocks.Examples.SendingTest do
  use ExUnit.Case, async: true
  alias Slackblocks.Blocks.{DataTable, Markdown, Plan, Section, TaskCard}
  alias Slackblocks.Examples.{HttpServer, Sending}
  alias Slackblocks.Objects.{RawNumber, RawText}

  alias Slackblocks.{Attachment, Message, WebhookMessage}

  for sender <- [:community, :direct] do
    test "#{sender} sends actual authenticated JSON with complete native values" do
      url = HttpServer.start([%{}])
      message = payload()

      assert {:ok, %{"ok" => true, "ts" => "1.0"}} =
               apply(Sending, unquote(sender), [message, "test-token", [base_url: url <> "/api"]])

      assert_receive {:request, :POST, "/api/chat.postMessage", headers, body}
      assert headers["authorization"] == "Bearer test-token"
      assert headers["content-type"] =~ "application/json"
      wire = Jason.decode!(body)
      assert wire === Slackblocks.to_map!(message)

      assert wire["metadata"]["event_payload"] === %{
               "object" => %{},
               "array" => [],
               "null" => nil,
               "false" => false,
               "zero" => 0,
               "unicode" => "👋 café",
               "blocks" => [%{"type" => "unknown"}]
             }

      assert get_in(wire, ["blocks", Access.at(1), "tasks", Access.at(0), "status"]) == "pending"
      refute Map.has_key?(get_in(wire, ["blocks", Access.at(1), "tasks", Access.at(0)]), "type")
      refute_receive {:request, _, _, _, _}, 20
    end
  end

  test "HTTP failures, Slack failures, malformed JSON and missing ok are distinct" do
    for {response, expected} <- [
          {%{status: 500, body: "failure"}, {:http, 500, "failure"}},
          {%{status: 302, body: "redirect", headers: %{"location" => "/duplicate"}},
           {:http, 302, "redirect"}},
          {%{body: ~s({"ok":false,"error":"not_in_channel"})}, {:slack, "not_in_channel"}},
          {%{body: "not json"}, :invalid_response},
          {%{body: ~s({"message":"oops"})}, :invalid_response}
        ] do
      url = HttpServer.start([response])
      assert {:error, ^expected} = Sending.direct(payload(), "test-token", base_url: url)
      assert_receive {:request, :POST, _, _, _}
      refute_receive {:request, _, _, _, _}, 20
    end
  end

  test "429 retries observe Retry-After, have one owner and stop after three requests" do
    owner = self()
    sleep = fn delay -> send(owner, {:sleep, delay}) end
    limited = %{status: 429, headers: %{"retry-after" => "1"}, body: "rate limited"}
    url = HttpServer.start([limited, limited, %{}])
    assert {:ok, _} = Sending.community(payload(), "test-token", base_url: url, sleep: sleep)
    for _ <- 1..3, do: assert_receive({:request, :POST, _, _, _})
    for _ <- 1..2, do: assert_receive({:sleep, 1_000})
    refute_receive {:request, _, _, _, _}, 20
    refute_receive {:sleep, _}, 20

    url = HttpServer.start(List.duplicate(limited, 3))

    assert {:error, {:http, 429, "rate limited"}} =
             Sending.direct(payload(), "test-token", base_url: url, sleep: sleep)

    for _ <- 1..3, do: assert_receive({:request, :POST, _, _, _})
    for _ <- 1..2, do: assert_receive({:sleep, 1_000})
    refute_receive {:request, _, _, _, _}, 20
    refute_receive {:sleep, _}, 20
  end

  test "zero Retry-After uses the real sleep path without a long wait" do
    limited = %{status: 429, headers: %{"retry-after" => "0"}}
    url = HttpServer.start([limited, %{}])
    started = System.monotonic_time(:millisecond)
    assert {:ok, _} = Sending.direct(payload(), "test-token", base_url: url)
    assert System.monotonic_time(:millisecond) - started < 2_000
    for _ <- 1..2, do: assert_receive({:request, :POST, _, _, _})
    refute_receive {:request, _, _, _, _}, 20
  end

  test "unbounded, missing or malformed retry delays cause no early retry" do
    for headers <- [
          %{},
          %{"retry-after" => "3"},
          %{"retry-after" => "-1"},
          %{"retry-after" => "soon"},
          %{"retry-after" => "1.5"}
        ] do
      url = HttpServer.start([%{status: 429, headers: headers, body: "limited"}])

      assert {:error, {:http, 429, "limited"}} =
               Sending.direct(payload(), "test-token",
                 base_url: url,
                 sleep: fn _ -> flunk("must not sleep") end
               )

      assert_receive {:request, :POST, _, _, _}
      refute_receive {:request, _, _, _, _}, 20
    end
  end

  test "timeouts return without retrying a potentially accepted POST" do
    url = HttpServer.start([%{delay: 200}])
    started = System.monotonic_time(:millisecond)

    assert {:error, %{reason: :timeout} = error} =
             Sending.direct(payload(), "test-token", base_url: url, receive_timeout: 20)

    assert is_exception(error)
    assert System.monotonic_time(:millisecond) - started < 2_000
    assert_receive {:request, :POST, _, _, _}
    refute_receive {:request, _, _, _, _}, 50
  end

  test "webhooks use their plain-text success contract and send no bearer token" do
    message = WebhookMessage.new!(text: "Webhook", blocks: [Section.new!(text: "Hello 👋")])
    url = HttpServer.start([%{body: "ok", headers: %{"content-type" => "text/plain"}}])
    assert :ok = Sending.webhook(message, url <> "/hook")
    assert_receive {:request, :POST, "/hook", headers, body}
    refute Map.has_key?(headers, "authorization")
    assert Jason.decode!(body) === Slackblocks.to_map!(message)

    for response <- [%{status: 403, body: "forbidden"}, %{body: "unexpected"}] do
      url = HttpServer.start([response])
      assert {:error, _} = Sending.webhook(message, url)
      assert_receive {:request, :POST, _, _, _}
    end
  end

  test "invalid models fail before sending, and the Phoenix boundary is plain JSON data" do
    message = %{payload() | channel: ""}

    assert {:error, %Slackblocks.ValidationError{}} =
             Sending.direct(message, "test-token", base_url: "http://127.0.0.1:1")

    assert {:error, %Slackblocks.ValidationError{}} =
             Sending.webhook(message, "http://127.0.0.1:1")

    assert %{
             "response_type" => "ephemeral",
             "text" => "Request received",
             "blocks" => [%{"type" => "section"}]
           } = Sending.response_payload()

    assert Jason.encode!(Sending.response_payload()) |> Jason.decode!() ==
             Sending.response_payload()
  end

  test "the documented Req examples send their actual displayed code" do
    readme = File.read!(Path.expand("../README.md", __DIR__))
    snippets = Regex.scan(~r/```elixir\n(.*?)\n```/s, readme, capture: :all_but_first)
    assert length(snippets) == 2

    for [code] <- snippets do
      refute code =~ "https://",
             "examples must receive the base URL from their documented context"

      url = HttpServer.start([%{}])
      message = payload()

      assert {{:ok, %Req.Response{status: 200, body: %{"ok" => true}}}, _binding} =
               Code.eval_string(code,
                 token: "test-token",
                 message: message,
                 base_url: url <> "/api"
               )

      assert_receive {:request, :POST, "/api/chat.postMessage", headers, body}
      assert headers["authorization"] == "Bearer test-token"
      assert Jason.decode!(body) === Slackblocks.to_map!(message)
    end
  end

  test "Ecto and Decimal coexist with checked JSON decoding" do
    assert Decimal.equal?(Ecto.Type.cast(:decimal, "0.1") |> elem(1), Decimal.new("0.1"))

    assert Slackblocks.decode!(RawNumber, ~s({"type":"raw_number","text":"0.1","value":0.1})).value ===
             0.1

    if expected = System.get_env("SLACKBLOCKS_EXPECTED_VERSION"),
      do: assert(Slackblocks.version() == expected)

    assert Slackblocks.version() == Application.spec(:slackblocks, :vsn) |> List.to_string()
  end

  defp payload do
    Message.new!(
      channel: "C123",
      text: "Fallback",
      mrkdwn: false,
      blocks: [
        Section.new!(text: "Hello 👋"),
        Plan.new!(
          title: "Deployment",
          tasks: [TaskCard.new!(task_id: "a", title: "Queued", status: :pending)]
        ),
        DataTable.new!(
          caption: "Counts",
          rows: [[RawText.new!(text: "Count")], [RawNumber.new!(text: "0", value: 0)]]
        ),
        Markdown.new!(text: "")
      ],
      attachments: [Attachment.new!(blocks: [Section.new!(text: "Attachment")], color: "good")],
      metadata: %{
        "event_type" => "integration_test",
        "event_payload" => %{
          "object" => %{},
          "array" => [],
          "null" => nil,
          "false" => false,
          "zero" => 0,
          "unicode" => "👋 café",
          "blocks" => [%{"type" => "unknown"}]
        }
      }
    )
  end
end
