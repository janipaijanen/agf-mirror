defmodule Agov.ValidateAdoption do
  @moduledoc false

  alias Agov.ValidatorUtil

  @mandatory ~w(contribution-auditor infosec-sentinel)

  def validate(root \\ nil) do
    target = root || ValidatorUtil.root_dir()
    agents_dir = Path.join(target, "agents")

    errors =
      []
      |> require_path(File.dir?(agents_dir), "agents/ directory is missing", "agents")
      |> require_agent_files(agents_dir)
      |> require_mandatory_agents(agents_dir)
      |> require_governance_doc(target)
      |> require_path(
        File.exists?(Path.join(agents_dir, "MANIFEST.yaml")),
        "agents/MANIFEST.yaml is missing",
        "agents/MANIFEST.yaml"
      )

    if errors == [], do: {:ok, []}, else: {:error, Enum.reverse(errors)}
  end

  defp require_path(errors, true, _message, _location), do: errors

  defp require_path(errors, false, message, location),
    do: [%{message: message, location: location} | errors]

  defp require_governance_doc(errors, root) do
    exists? =
      File.exists?(Path.join(root, "GOVERNANCE.md")) or
        File.exists?(Path.join(root, "framework.md"))

    require_path(
      errors,
      exists?,
      "GOVERNANCE.md or framework.md is missing",
      "GOVERNANCE.md"
    )
  end

  defp require_agent_files(errors, agents_dir) do
    files = agent_files(agents_dir)
    require_path(errors, files != [], "agents/ has no agent YAML files", "agents")
  end

  defp require_mandatory_agents(errors, agents_dir) do
    present =
      agents_dir
      |> agent_files()
      |> Enum.flat_map(&agent_names/1)
      |> MapSet.new()

    @mandatory
    |> Enum.reject(&MapSet.member?(present, &1))
    |> Enum.reduce(errors, fn agent, acc ->
      [%{message: "mandatory agent missing: #{agent}", location: "agents"} | acc]
    end)
  end

  defp agent_files(agents_dir) do
    agents_dir
    |> ValidatorUtil.yaml_files()
    |> Enum.reject(&(Path.basename(&1) == "MANIFEST.yaml"))
  end

  defp agent_names(path) do
    case ValidatorUtil.read_yaml_file(path) do
      {:ok, data, _content} ->
        id = get_in(data, ["agent", "id"])
        name = get_in(data, ["agent", "name"])
        [name, id_segment(id)] |> Enum.reject(&is_nil/1)

      {:error, _errors} ->
        []
    end
  end

  defp id_segment(id) when is_binary(id), do: id |> String.split(":") |> Enum.at(1)
  defp id_segment(_id), do: nil
end
