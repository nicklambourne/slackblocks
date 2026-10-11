{options, _, invalid} = OptionParser.parse(System.argv(), strict: [profile: :string])
if invalid != [], do: raise("unknown checker options: #{inspect(invalid)}")
profile = Keyword.get(options, :profile, "full")
unless profile in ["full", "runtime"], do: raise("profile must be full or runtime")

run = fn command, args ->
  IO.puts("\nChecking: #{command} #{Enum.join(args, " ")}")
  {_, status} = System.cmd(command, args, into: IO.stream(:stdio, :line))
  if status != 0, do: System.halt(status)
end

python =
  System.get_env("SLACKBLOCKS_PYTHON") || System.find_executable("python3.14") ||
    System.find_executable("python3") || System.find_executable("python") ||
    raise("Python 3.11+ is required")

if profile == "full" do
  run.(python, ["generator/generate.py", "--check"])
  run.(python, ["generator/test_generate.py"])
  run.("mix", ["format", "--check-formatted"])
  run.("mix", ["credo", "--strict"])
  run.("mix", ["docs", "--warnings-as-errors"])
end

run.("mix", ["compile", "--warnings-as-errors"])
run.("mix", ["test", "--warnings-as-errors"])
run.(python, ["tools/package_check.py"])
IO.puts("Elixir #{profile} checks passed. Remaining train gates are tracked in design/plan.md.")
