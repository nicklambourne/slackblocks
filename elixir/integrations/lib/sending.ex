defmodule Slackblocks.Examples.Sending do
  @moduledoc """
  Executable application examples. HTTP clients and retry policy stay outside the core package.

  Only explicit HTTP 429 responses are retried, at most twice, with a maximum
  Retry-After of two seconds per retry. Timeouts and other ambiguous POST failures
  return immediately. Req's automatic retry and redirect handling are disabled.
  """
  alias Slackblocks.Blocks.Section
  alias Slackblocks.MessageResponse

  @doc "Uses the community client's configured Req request and sends a validated JSON body."
  def community(message, token, options \\ []),
    do: send_api(Slack.API.client(token), message, options)

  @doc "Uses Req directly; no Slack SDK is needed."
  def direct(message, token, options \\ []) do
    Req.new(base_url: "https://slack.com/api", auth: {:bearer, token})
    |> send_api(message, options)
  end

  @doc "Sends a WebhookMessage to an incoming webhook; success is the plain text `ok`."
  def webhook(message, url, options \\ []) do
    with {:ok, body} <- Slackblocks.to_map(message) do
      request =
        Req.new(
          url: url,
          json: body,
          retry: false,
          redirect: false,
          decode_body: false,
          receive_timeout: Keyword.get(options, :receive_timeout, 5_000)
        )

      request |> post(2, Keyword.get(options, :sleep, &Process.sleep/1)) |> result(:webhook)
    end
  end

  @doc "Pure response data suitable for a Phoenix JSON response after an interaction. Acknowledgement timing belongs to the application."
  def response_payload do
    MessageResponse.new!(
      response_type: :ephemeral,
      text: "Request received",
      blocks: [Section.new!(text: "Working on your request.")]
    )
    |> Slackblocks.to_map!()
  end

  defp send_api(request, message, options) do
    with {:ok, body} <- Slackblocks.to_map(message) do
      request =
        Req.merge(request,
          base_url: Keyword.get(options, :base_url, "https://slack.com/api"),
          url: "chat.postMessage",
          json: body,
          retry: false,
          redirect: false,
          decode_body: false,
          receive_timeout: Keyword.get(options, :receive_timeout, 5_000)
        )

      request |> post(2, Keyword.get(options, :sleep, &Process.sleep/1)) |> result(:api)
    end
  end

  defp post(request, remaining, sleep) do
    response = Req.post(request)

    case response do
      {:ok, %Req.Response{status: 429} = limited} when remaining > 0 ->
        case retry_after(limited) do
          {:ok, milliseconds} ->
            sleep.(milliseconds)
            post(request, remaining - 1, sleep)

          :error ->
            response
        end

      _ ->
        response
    end
  end

  defp retry_after(response) do
    case Req.Response.get_header(response, "retry-after") do
      [value] ->
        case Integer.parse(value) do
          {seconds, ""} when seconds in 0..2 -> {:ok, seconds * 1_000}
          _ -> :error
        end

      _ ->
        :error
    end
  end

  defp result({:error, error}, _), do: {:error, error}

  defp result({:ok, %Req.Response{status: status, body: body}}, _) when status != 200,
    do: {:error, {:http, status, body}}

  defp result({:ok, %Req.Response{body: body}}, :webhook) do
    if String.trim(body) == "ok", do: :ok, else: {:error, :invalid_response}
  end

  defp result({:ok, %Req.Response{body: body}}, :api) do
    case Jason.decode(body) do
      {:ok, %{"ok" => true} = value} -> {:ok, value}
      {:ok, %{"ok" => false, "error" => error}} -> {:error, {:slack, error}}
      _ -> {:error, :invalid_response}
    end
  end
end
