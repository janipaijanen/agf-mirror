defmodule Agov.MixProject do
  use Mix.Project

  def project do
    [
      app: :agov,
      version: "0.1.0",
      elixir: "~> 1.20",
      start_permanent: Mix.env() == :prod,
      escript: [main_module: Agov.CLI],
      releases: [
        agov: [
          include_erts: true,
          include_executables_for: [:unix],
          strip_beams: true,
          overlays: ["rel/overlays"]
        ]
      ],
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:jason, "~> 1.4"},
      {:ex_json_schema, "~> 0.10"},
      {:yaml_elixir, "~> 2.9"},
      {:ulid, "~> 0.2.0"}
    ]
  end
end
