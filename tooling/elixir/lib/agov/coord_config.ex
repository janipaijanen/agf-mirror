defmodule Agov.CoordConfig do
  @moduledoc false

  @config_filename "agov-config.yaml"

  def load(coord_dir) do
    path = Path.join(coord_dir, @config_filename)

    case File.read(path) do
      {:error, :enoent} ->
        %{from_aliases: %{}, role: nil}

      {:ok, content} ->
        data = YamlElixir.read_from_string!(content)
        aliases = data |> Map.get("from_aliases") |> normalize_aliases()
        role = data |> Map.get("current_role") |> normalize_role()

        %{from_aliases: aliases, role: role}
    end
  end

  defp normalize_aliases(map) when is_map(map) do
    map
    |> Enum.filter(fn {k, v} -> is_binary(k) and is_binary(v) end)
    |> Map.new()
  end

  defp normalize_aliases(_), do: %{}

  defp normalize_role(role) when is_binary(role) and role != "", do: role
  defp normalize_role(_), do: nil
end
