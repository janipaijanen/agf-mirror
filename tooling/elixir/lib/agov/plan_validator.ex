defmodule Agov.PlanValidator do
  @moduledoc false

  @required_roles ~w(infosec-sentinel contribution-auditor)

  def validate_file(path) do
    with {:ok, content} <- read_file(path),
         {:ok, yaml_str} <- Agov.CoordValidator.extract_frontmatter(content),
         {:ok, data} <- parse_yaml(yaml_str),
         :ok <- validate_against_schema(data),
         :ok <- validate_semantics(data) do
      {:ok, data}
    end
  end

  def required_roles, do: @required_roles

  def validate_semantics(data) do
    entries = Map.get(data, "advisory_consultation", [])
    roles = Enum.map(entries, &Map.get(&1, "role"))

    errors =
      []
      |> add_missing_role_errors(roles)
      |> add_unknown_role_errors(roles)
      |> add_duplicate_role_errors(roles)

    case errors do
      [] -> :ok
      _ -> {:error, Enum.reverse(errors)}
    end
  end

  defp add_missing_role_errors(errors, roles) do
    @required_roles
    |> Enum.reject(&(&1 in roles))
    |> Enum.reduce(errors, fn role, acc ->
      [{"missing required advisory role: #{role}", "#/advisory_consultation"} | acc]
    end)
  end

  defp add_unknown_role_errors(errors, roles) do
    roles
    |> Enum.reject(&(&1 in @required_roles))
    |> Enum.uniq()
    |> Enum.reduce(errors, fn role, acc ->
      [{"unknown advisory role: #{role}", "#/advisory_consultation"} | acc]
    end)
  end

  defp add_duplicate_role_errors(errors, roles) do
    roles
    |> Enum.frequencies()
    |> Enum.filter(fn {_role, count} -> count > 1 end)
    |> Enum.reduce(errors, fn {role, _count}, acc ->
      [{"duplicate advisory role: #{role}", "#/advisory_consultation"} | acc]
    end)
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
    dir = System.get_env("AGOV_SCHEMA_DIR", "../../spec/schemas")
    Path.join(dir, "gate1-plan.schema.json")
  end

  defp load_schema do
    schema_path()
    |> File.read!()
    |> Jason.decode!()
    |> ExJsonSchema.Schema.resolve()
  end
end
