defmodule Agov.CoordValidator do
  @moduledoc false

  def validate_file(path) do
    with {:ok, content} <- read_file(path),
         {:ok, yaml_str} <- extract_frontmatter(content),
         {:ok, data} <- parse_yaml(yaml_str),
         :ok <- validate_against_schema(data) do
      {:ok, data}
    end
  end

  def extract_frontmatter(content) do
    case Regex.run(~r/\A---\n(.*?)\n---(?:\n|\z)/s, content, capture: :all_but_first) do
      [yaml_str] -> {:ok, yaml_str}
      nil -> {:error, :no_frontmatter}
    end
  end

  defp read_file(path) do
    case File.read(path) do
      {:ok, content} -> {:ok, content}
      {:error, :enoent} -> {:error, :file_not_found}
      {:error, reason} -> {:error, reason}
    end
  end

  defp parse_yaml(yaml_str) do
    case YamlElixir.read_from_string(yaml_str) do
      {:ok, data} -> {:ok, data}
      {:error, reason} -> {:error, {:yaml_parse_error, reason}}
    end
  end

  defp validate_against_schema(data) do
    schema = load_schema()
    case ExJsonSchema.Validator.validate(schema, data) do
      :ok -> :ok
      {:error, errors} -> {:error, errors}
    end
  end

  defp schema_path do
    dir = System.get_env("AGOV_SCHEMA_DIR", "../../agf/schemas")
    Path.join(dir, "coordination-message.schema.json")
  end

  defp load_schema do
    schema_path()
    |> File.read!()
    |> Jason.decode!()
    |> ExJsonSchema.Schema.resolve()
  end
end
