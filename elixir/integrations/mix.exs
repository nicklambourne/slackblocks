defmodule SlackblocksIntegrations.MixProject do
  use Mix.Project

  def project do
    [
      app: :slackblocks_integrations,
      version: "0.0.0",
      elixir: ">= 1.16.3 and < 2.0.0",
      elixirc_paths: if(Mix.env() == :test, do: ["lib", "test/support"], else: ["lib"]),
      deps: [
        {:slackblocks, "== 3.0.0", path: ".."},
        {:slack_elixir, "~> 1.2.2"},
        {:req, ">= 0.5.0 and < 1.0.0"},
        {:ecto, "~> 3.14.0"}
      ]
    ]
  end

  def application, do: []
end
