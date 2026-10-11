defmodule Slackblocks.MixProject do
  use Mix.Project

  def project do
    [
      app: :slackblocks,
      version: "2.8.0-dev",
      elixir: ">= 1.16.3 and < 2.0.0",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      elixirc_paths: if(Mix.env() == :test, do: ["lib", "test/support"], else: ["lib"]),
      test_coverage: [summary: false],
      dialyzer: [
        plt_local_path: System.get_env("SLACKBLOCKS_PLT_PATH", "_build/dev/plts"),
        plt_core_path: System.get_env("SLACKBLOCKS_PLT_PATH", "_build/dev/plts")
      ],
      description: "Native, validated Slack Block Kit values for Elixir and Erlang",
      source_url: "https://github.com/nicklambourne/slackblocks",
      package: [
        licenses: ["MIT", "BSD-3-Clause"],
        links: %{"GitHub" => "https://github.com/nicklambourne/slackblocks"},
        files: ~w(lib mix.exs README.md CHANGELOG.md LICENSE LICENSE.BSD-3-Clause)
      ],
      docs: [
        main: "readme",
        extras: ["README.md", "CHANGELOG.md"],
        source_ref: System.get_env("SLACKBLOCKS_SOURCE_REF", "master"),
        source_url_pattern:
          "https://github.com/nicklambourne/slackblocks/blob/#{System.get_env("SLACKBLOCKS_SOURCE_REF", "master")}/elixir/%{path}#L%{line}"
      ]
    ]
  end

  def application, do: []

  defp deps do
    [
      {:jason, ">= 1.4.5 and < 2.0.0"},
      {:decimal, "~> 2.4 or ~> 3.0"},
      {:ex_doc, "~> 0.38.0", only: :dev, runtime: false},
      {:credo, "~> 1.7.0", only: [:dev, :test], runtime: false},
      {:dialyxir, "~> 1.4", only: :dev, runtime: false},
      {:stream_data, "~> 1.2", only: :test}
    ]
  end
end
