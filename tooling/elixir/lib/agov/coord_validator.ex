defmodule Agov.CoordValidator do
  @moduledoc false

  # Backing rule for blocking authority: agf/rules/always/tool-authority-boundary-rule.md
  # Schema: agf/schemas/coordination-message.schema.json

  def validate_file(path) do
    with {:ok, content} <- read_file(path),
         {:ok, yaml_str} <- extract_frontmatter(content),
         {:ok, data} <- parse_yaml(yaml_str),
         :ok <- validate_against_schema(data) do
      {:ok, data}
    end
  end

  @doc """
  Returns a list of advisory warning strings for a parsed coordination message.

  Advisory findings do not affect exit codes or block work. They surface
  recommended practices that are not enforced by the schema.
  """
  def advisory_findings(data) when is_map(data) do
    []
    |> check_deprecated_fields(data)
    |> check_model_field(data)
  end

  defp check_deprecated_fields(findings, data) do
    raw = load_raw_schema()
    properties = Map.get(raw, "properties", %{})

    deprecated =
      properties
      |> Enum.filter(fn {_name, prop} -> prop["x-deprecated"] == true end)
      |> Enum.map(fn {name, _prop} -> name end)

    Enum.reduce(deprecated, findings, fn field, acc ->
      if Map.has_key?(data, field) do
        ["'#{field}' is deprecated; remove it — causal ordering belongs in references-to and timestamp" | acc]
      else
        acc
      end
    end)
  end

  defp check_model_field(findings, data) do
    case Map.get(data, "model") do
      nil ->
        ["recommended field 'model' not set; provenance tracking is weaker without it" | findings]

      _ ->
        findings
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

  defp load_raw_schema do
    schema_path()
    |> File.read!()
    |> Jason.decode!()
  end
end
