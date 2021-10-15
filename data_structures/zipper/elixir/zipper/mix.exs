defmodule Zipper.MixProject do
  use Mix.Project

  def project do
    [
      app: :zipper,
      version: "0.1.0",
      elixir: "~> 1.12",
      start_permanent: Mix.env() == :prod,
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
      {:binary_tree, path: "../../../binary_tree/elixir/binary_tree"},
      {:ternary_tree, path: "../../../ternary_tree/elixir/ternary_tree"}
    ]
  end
end
