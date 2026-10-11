{options, _, invalid} = OptionParser.parse(System.argv(), strict: [profile: :string])
if invalid != [], do: raise("unknown checker options: #{inspect(invalid)}")
profile = Keyword.get(options, :profile, "full")

unless profile in ["full", "runtime", "native"],
  do: raise("profile must be full, runtime or native")

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

  ruff =
    System.get_env("SLACKBLOCKS_RUFF") || System.find_executable("ruff") ||
      raise("Ruff 0.15.12 is required; install it with pip or uv")

  run.(ruff, ["format", "--check", "generator", "tools"])
  run.(ruff, ["check", "generator", "tools"])
  run.("mix", ["format", "--check-formatted"])

  {_, format_status} =
    System.cmd("mix", ["format", "--check-formatted"],
      cd: "integrations",
      into: IO.stream(:stdio, :line)
    )

  if format_status != 0, do: System.halt(format_status)
  run.("mix", ["credo", "--strict"])
  run.("mix", ["credo", "--strict", "integrations"])
  run.("mix", ["docs", "--warnings-as-errors"])
  run.("mix", ["dialyzer"])
  run.(python, ["tools/docs_check.py"])
end

run.("mix", ["compile", "--warnings-as-errors"])

if profile == "full" do
  run.("mix", ["test", "--cover", "--export-coverage", "slackblocks", "--warnings-as-errors"])
  previous_env = System.get_env("MIX_ENV")
  System.put_env("MIX_ENV", "test")
  run.("mix", ["run", "tools/coverage.exs"])
  if previous_env, do: System.put_env("MIX_ENV", previous_env), else: System.delete_env("MIX_ENV")
  run.(python, ["tools/quality_mutations.py"])
else
  run.("mix", ["test", "--warnings-as-errors"])
end

run.(python, ["tools/package_check.py"])
if profile == "full", do: run.(python, ["tools/erlang_check.py"])

if profile != "native" do
  profiles = if profile == "full", do: ["locked", "lower", "latest"], else: ["locked"]
  Enum.each(profiles, &run.(python, ["tools/integration_check.py", "--profile", &1]))
end

IO.puts("Elixir #{profile} checks passed. Remaining train gates are tracked in design/plan.md.")
