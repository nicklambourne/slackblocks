cases = IO.read(:stdio, :eof) |> Jason.decode!()

for %{"label" => label, "code" => code, "expected" => expected} <- cases do
  try do
    {value, _binding} = Code.eval_string(code, [], file: label)

    if expected != nil and value != expected do
      raise "JSON differs: #{inspect(value)} != #{inspect(expected)}"
    end
  rescue
    error ->
      IO.puts(:stderr, "#{label}: #{Exception.format(:error, error, __STACKTRACE__)}")
      System.halt(1)
  end
end

IO.puts("Executed #{length(cases)} literal Elixir README/guide/reusable examples.")
