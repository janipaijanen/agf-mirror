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
  def advisory_findings(data, path \\ nil)

  def advisory_findings(data, path) when is_map(data) do
    []
    |> check_deprecated_fields(data)
    |> check_model_field(data)
    |> check_agent_manual_id(data)
    |> check_readable_id_drift(data, path)
  end

  def duplicate_id_errors(file_data) do
    file_data
    |> Enum.filter(fn {_file, data} -> is_map(data) and is_binary(data["id"]) end)
    |> Enum.group_by(fn {_file, data} -> data["id"] end, fn {file, _data} -> file end)
    |> Enum.filter(fn {_id, files} -> length(files) > 1 end)
    |> Enum.flat_map(fn {id, files} ->
      sorted = Enum.sort(files)
      [first | rest] = sorted

      Enum.map(rest, fn file ->
        {file, "duplicate coordination id '#{id}' also used by #{first}"}
      end)
    end)
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
        [
          "'#{field}' is deprecated; remove it — causal ordering belongs in references-to and timestamp"
          | acc
        ]
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

  defp check_agent_manual_id(findings, data) do
    id = data["id"]
    from = data["from"]

    cond do
      !is_binary(id) or !is_binary(from) ->
        findings

      Agov.CoordIdentity.readable_id?(id) or Agov.CoordIdentity.ulid?(id) ->
        findings

      ai_agent_role?(from) ->
        [
          "AI agent role '#{from}' used manual coordination id '#{id}'; use agov coord brief --id-format readable unless tooling is unavailable"
          | findings
        ]

      true ->
        findings
    end
  end

  defp check_readable_id_drift(findings, data, path) do
    id = data["id"]

    if Agov.CoordIdentity.readable_id?(id) do
      findings
      |> check_readable_id_role(data, id)
      |> check_readable_id_timestamp(data, id)
      |> check_readable_filename(data, id, path)
    else
      findings
    end
  end

  defp check_readable_id_role(findings, data, id) do
    with {:ok, %{role: role}} <- Agov.CoordIdentity.readable_parts(id),
         from when is_binary(from) <- data["from"],
         expected <- Agov.CoordIdentity.role_slug(from),
         false <- role == expected do
      [
        "readable id role '#{role}' differs from from '#{from}' (expected slug '#{expected}')"
        | findings
      ]
    else
      _ -> findings
    end
  end

  defp check_readable_id_timestamp(findings, data, id) do
    with {:ok, %{timestamp: id_timestamp}} <- Agov.CoordIdentity.readable_parts(id),
         timestamp when is_binary(timestamp) <- data["timestamp"],
         {:ok, dt, _offset} <- DateTime.from_iso8601(timestamp),
         frontmatter_timestamp <- Agov.CoordIdentity.basic_utc_timestamp(dt),
         false <- id_timestamp == frontmatter_timestamp do
      [
        "readable id timestamp '#{id_timestamp}' differs from frontmatter timestamp '#{frontmatter_timestamp}'"
        | findings
      ]
    else
      _ -> findings
    end
  end

  defp check_readable_filename(findings, _data, _id, nil), do: findings

  defp check_readable_filename(findings, _data, id, path) do
    basename = Path.basename(path)
    expected = id <> ".md"

    cond do
      not String.ends_with?(basename, ".md") ->
        findings

      basename == expected ->
        findings

      String.starts_with?(basename, id <> "-") ->
        [
          "readable filename '#{basename}' repeats sender suffix; expected '#{expected}'"
          | findings
        ]

      true ->
        ["readable filename '#{basename}' does not match expected '#{expected}'" | findings]
    end
  end

  defp ai_agent_role?(role) do
    case Agov.ManifestReader.load(manifest_path()) do
      {:ok, agents} ->
        case Map.get(agents, role) do
          %{played_by: played_by} when is_binary(played_by) ->
            played_by
            |> String.downcase()
            |> then(&(String.contains?(&1, "ai") or String.contains?(&1, "claude")))

          _ ->
            false
        end

      _ ->
        false
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

  defp manifest_path do
    System.get_env("AGOV_MANIFEST_PATH", "../../agf/agents/MANIFEST.yaml")
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
