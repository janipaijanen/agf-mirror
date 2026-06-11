defmodule Agov.CoordConfig do
  @moduledoc false

  @config_filename "agov-config.yaml"

  def load(coord_dir) do
    path = Path.join(coord_dir, @config_filename)

    case File.read(path) do
      {:error, :enoent} ->
        %{from_aliases: %{}}

      {:ok, content} ->
        aliases =
          content
          |> YamlElixir.read_from_string!()
          |> Map.get("from_aliases")
          |> normalize_aliases()

        %{from_aliases: aliases}
    end
  end

  defp normalize_aliases(map) when is_map(map) do
    map
    |> Enum.filter(fn {k, v} -> is_binary(k) and is_binary(v) end)
    |> Map.new()
  end

  defp normalize_aliases(_), do: %{}
end
