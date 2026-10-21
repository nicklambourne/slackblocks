config = Mix.Project.config()
Application.load(:hex)

IO.puts(
  Jason.encode!(%{
    package: Atom.to_string(config[:app]),
    version: config[:version],
    runtime_version: Slackblocks.version(),
    spec_version: Slackblocks.spec_version(),
    elixir: config[:elixir],
    licenses: config[:package][:licenses],
    dependencies:
      Map.new(config[:deps], fn
        {name, requirement} -> {name, requirement}
        {name, requirement, _options} -> {name, requirement}
      end)
      |> Map.take([:jason, :decimal]),
    compiler: System.version(),
    hex: to_string(Application.spec(:hex, :vsn)),
    otp: System.otp_release()
  })
)
