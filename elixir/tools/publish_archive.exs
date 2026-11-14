# Upload exactly the reproduced and tested archive, never a second implicit build.
[archive, expected_checksum] = System.argv()
Hex.start()
bytes = File.read!(archive)
checksum = :crypto.hash(:sha256, bytes) |> Base.encode16(case: :lower)
unless checksum == expected_checksum, do: Mix.raise("Tested Hex archive checksum differs")

case Hex.API.Release.publish(nil, bytes, [], fn _ -> :ok end, false) do
  {:ok, {status, _, _}} when status in 200..299 ->
    IO.puts("Published tested Hex archive: " <> checksum)

  _ ->
    Mix.raise("Hex archive upload failed; inspect registry identity before retrying")
end
