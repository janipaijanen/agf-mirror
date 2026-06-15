defmodule Agov.ValidateAdoptionTest do
  use ExUnit.Case, async: true

  test "accepts a root with mandatory agents, manifest, and governance document" do
    root = adoption_root()

    assert {:ok, []} = Agov.ValidateAdoption.validate(root)
  end

  test "accepts framework.md as the AGF product governance document" do
    root = adoption_root(governance_file: "framework.md")

    assert {:ok, []} = Agov.ValidateAdoption.validate(root)
  end

  test "rejects roots missing mandatory agents" do
    root = tmp_dir("adoption-missing")
    agents_dir = Path.join(root, "agents")
    File.mkdir_p!(agents_dir)
    File.write!(Path.join(root, "GOVERNANCE.md"), "# Governance\n")
    File.write!(Path.join(agents_dir, "MANIFEST.yaml"), "agents: []\n")

    assert {:error, errors} = Agov.ValidateAdoption.validate(root)
    assert Enum.any?(errors, &String.contains?(&1.message, "mandatory agent missing"))
  end

  defp adoption_root(opts \\ []) do
    root = tmp_dir("adoption")
    agents_dir = Path.join(root, "agents")
    File.mkdir_p!(agents_dir)

    File.write!(
      Path.join(root, Keyword.get(opts, :governance_file, "GOVERNANCE.md")),
      "# Governance\n"
    )

    File.write!(Path.join(agents_dir, "MANIFEST.yaml"), "agents: []\n")
    write_agent(agents_dir, "contribution-auditor")
    write_agent(agents_dir, "infosec-sentinel")

    root
  end

  defp write_agent(dir, name) do
    File.write!(Path.join(dir, "#{name}.yaml"), """
    agent:
      name: #{name}
      id: agents-govern:#{name}:0.1.0
    """)
  end

  defp tmp_dir(name) do
    path = Path.join(System.tmp_dir!(), "agov-#{name}-#{System.unique_integer([:positive])}")
    File.mkdir_p!(path)
    ExUnit.Callbacks.on_exit(fn -> File.rm_rf!(path) end)
    path
  end
end
