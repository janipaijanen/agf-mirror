defmodule Agov.ValidatorUtil do
  @moduledoc false

  def read_yaml_file(path) do
    with {:ok, content} <- File.read(path),
         {:ok, data} <- YamlElixir.read_from_string(content) do
      {:ok, data, content}
    else
      {:error, :enoent} ->
        {:error, [%{message: "file not found", location: path}]}

      {:error, reason} ->
        {:error, [%{message: "YAML parse/read error: #{inspect(reason)}", location: path}]}
    end
  end

  def validate_schema(data, schema_name, schema_dir \\ schema_dir()) do
    schema_path = Path.join(schema_dir, schema_name)

    with {:ok, content} <- File.read(schema_path),
         {:ok, decoded} <- Jason.decode(content) do
      schema = ExJsonSchema.Schema.resolve(decoded)

      case ExJsonSchema.Validator.validate(schema, data) do
        :ok ->
          []

        {:error, errors} ->
          Enum.map(errors, fn {message, location} ->
            %{message: message, location: location}
          end)
      end
    else
      {:error, reason} ->
        [%{message: "schema load error: #{inspect(reason)}", location: schema_path}]
    end
  end

  def yaml_files(dir) do
    case File.ls(dir) do
      {:ok, files} ->
        files
        |> Enum.filter(&(String.ends_with?(&1, ".yaml") or String.ends_with?(&1, ".yml")))
        |> Enum.sort()
        |> Enum.map(&Path.join(dir, &1))

      {:error, _reason} ->
        []
    end
  end

  def freetext(data, keys \\ ~w(description text summary recommendation)) do
    do_freetext(data, MapSet.new(keys), [])
    |> Enum.reverse()
  end

  defp do_freetext(%{} = map, keys, acc) do
    Enum.reduce(map, acc, fn {key, value}, inner ->
      cond do
        key in keys and is_binary(value) -> [value | inner]
        is_map(value) or is_list(value) -> do_freetext(value, keys, inner)
        true -> inner
      end
    end)
  end

  defp do_freetext(list, keys, acc) when is_list(list) do
    Enum.reduce(list, acc, &do_freetext(&1, keys, &2))
  end

  defp do_freetext(_value, _keys, acc), do: acc

  def schema_dir, do: System.get_env("AGOV_SCHEMA_DIR", "../../agf/schemas")
  def learnings_dir, do: System.get_env("AGOV_LEARNINGS_DIR", "../../agf/learnings")
  def agents_dir, do: System.get_env("AGOV_AGENTS_DIR", "../../agf/agents")
  def root_dir, do: System.get_env("AGOV_ROOT", "../../agf")
end
