defmodule Agov.ValidateAgents do
  @moduledoc false

  alias Agov.ValidatorUtil

  @tiers ~w(production governance cross-cutting publication-support meta)

  def validate_file(path) do
    with {:ok, data, _content} <- ValidatorUtil.read_yaml_file(path) do
      tier = get_in(data, ["agent", "tier"])

      errors =
        []
        |> Kernel.++(ValidatorUtil.validate_schema(data, "agent.schema.json"))
        |> maybe_add(tier not in @tiers, "agent.tier is missing or invalid", "agent/tier")

      if errors == [], do: {:ok, data}, else: {:error, errors}
    end
  end

  def validate_all(dir \\ nil) do
    target = dir || ValidatorUtil.agents_dir()

    target
    |> ValidatorUtil.yaml_files()
    |> Enum.reject(&(Path.basename(&1) == "MANIFEST.yaml"))
    |> Enum.map(fn file -> {file, validate_file(file)} end)
    |> split_results()
  end

  defp maybe_add(errors, true, message, location),
    do: [%{message: message, location: location} | errors]

  defp maybe_add(errors, false, _message, _location), do: errors

  defp split_results(results) do
    errors =
      results
      |> Enum.flat_map(fn
        {file, {:error, file_errors}} -> [%{file: file, errors: file_errors}]
        {_file, {:ok, _data}} -> []
      end)

    if errors == [], do: {:ok, Enum.map(results, &elem(&1, 0))}, else: {:error, errors}
  end
end
