defmodule Slackblocks.Examples.HttpServer do
  @moduledoc false
  import ExUnit.Callbacks, only: [on_exit: 1]

  def start(responses) do
    owner = self()

    {:ok, listener} =
      :gen_tcp.listen(0, [
        :binary,
        active: false,
        packet: :http_bin,
        ip: {127, 0, 0, 1},
        reuseaddr: true
      ])

    {:ok, {_, port}} = :inet.sockname(listener)
    {:ok, pid} = Task.start_link(fn -> Enum.each(responses, &serve(listener, owner, &1)) end)

    on_exit(fn ->
      :gen_tcp.close(listener)
      if Process.alive?(pid), do: Process.exit(pid, :shutdown)
    end)

    "http://127.0.0.1:#{port}"
  end

  defp serve(listener, owner, response) do
    {:ok, socket} = :gen_tcp.accept(listener, 5_000)
    {:ok, {:http_request, method, {:abs_path, path}, _}} = :gen_tcp.recv(socket, 0, 5_000)
    headers = headers(socket, %{})
    :ok = :inet.setopts(socket, packet: :raw)
    size = String.to_integer(Map.fetch!(headers, "content-length"))
    {:ok, body} = :gen_tcp.recv(socket, size, 5_000)
    send(owner, {:request, method, path, headers, body})
    if response[:delay], do: Process.sleep(response[:delay])
    body = Map.get(response, :body, ~s({"ok":true,"ts":"1.0"}))
    status = Map.get(response, :status, 200)

    headers =
      Map.merge(
        %{
          "content-type" => "application/json",
          "content-length" => Integer.to_string(byte_size(body)),
          "connection" => "close"
        },
        Map.get(response, :headers, %{})
      )

    lines = Enum.map(headers, fn {key, value} -> "#{key}: #{value}\r\n" end)
    :gen_tcp.send(socket, ["HTTP/1.1 #{status} Reply\r\n", lines, "\r\n", body])
    :gen_tcp.close(socket)
  end

  defp headers(socket, acc) do
    case :gen_tcp.recv(socket, 0, 5_000) do
      {:ok, :http_eoh} ->
        acc

      {:ok, {:http_header, _, key, _, value}} ->
        headers(socket, Map.put(acc, String.downcase(to_string(key)), value))
    end
  end
end
