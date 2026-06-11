defmodule Agov.ManifestReader do
  @moduledoc false

  def load(manifest_path) do
    case File.read(manifest_path) do
      {:error, :enoent} ->
        {:error, :enoent}

      {:ok, content} ->
        agents =
          content
          |> YamlElixir.read_from_string!()
          |> Map.get("agents", %{})
          |> normalize_agents()

        {:ok, agents}
    end
  end

  defp normalize_agents(map) when is_map(map) do
    map
    |> Enum.filter(fn {k, v} -> is_binary(k) and is_map(v) end)
    |> Map.new(fn {name, attrs} -> {name, %{status: Map.get(attrs, "status", "unknown")}} end)
  end

  defp normalize_agents(_), do: %{}
end
