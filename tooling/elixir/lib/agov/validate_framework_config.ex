defmodule Agov.ValidateFrameworkConfig do
  @moduledoc false

  alias Agov.ValidatorUtil

  def validate_file(path) do
    if File.exists?(path) do
      with {:ok, data, _content} <- ValidatorUtil.read_yaml_file(path) do
        root = path |> Path.dirname()
        version_path = Path.join(root, "VERSION")
        declared = data["contract_version"]
        repo_version = version(version_path)

        errors =
          []
          |> Kernel.++(ValidatorUtil.validate_schema(data, "framework.schema.json"))
          |> maybe_add(
            is_binary(declared) and is_binary(repo_version) and declared != repo_version,
            "contract_version does not match VERSION",
            "contract_version"
          )

        if errors == [], do: {:ok, data}, else: {:error, errors}
      end
    else
      {:ok, :skipped}
    end
  end

  def validate_default(root \\ nil) do
    target_root = root || ValidatorUtil.root_dir()
    validate_file(Path.join(target_root, "framework.yaml"))
  end

  defp version(path) do
    case File.read(path) do
      {:ok, content} -> String.trim(content)
      {:error, _reason} -> nil
    end
  end

  defp maybe_add(errors, true, message, location),
    do: [%{message: message, location: location} | errors]

  defp maybe_add(errors, false, _message, _location), do: errors
end
