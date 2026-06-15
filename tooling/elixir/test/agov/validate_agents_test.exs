defmodule Agov.ValidateAgentsTest do
  use ExUnit.Case, async: true

  @fixtures_dir Path.join([__DIR__, "..", "fixtures"])

  test "accepts a valid agent definition" do
    assert {:ok, _data} =
             Agov.ValidateAgents.validate_file(Path.join(@fixtures_dir, "valid_agent.yaml"))
  end

  test "rejects a missing tier" do
    assert {:error, errors} =
             Agov.ValidateAgents.validate_file(
               Path.join(@fixtures_dir, "invalid_agent_missing_tier.yaml")
             )

    assert Enum.any?(errors, &(&1.message == "agent.tier is missing or invalid"))
  end

  test "validate_all skips MANIFEST.yaml" do
    dir = tmp_dir("agents")
    File.cp!(Path.join(@fixtures_dir, "valid_agent.yaml"), Path.join(dir, "coder-tester.yaml"))
    File.write!(Path.join(dir, "MANIFEST.yaml"), "not: an agent definition\n")

    assert {:ok, [path]} = Agov.ValidateAgents.validate_all(dir)
    assert Path.basename(path) == "coder-tester.yaml"
  end

  defp tmp_dir(name) do
    path = Path.join(System.tmp_dir!(), "agov-#{name}-#{System.unique_integer([:positive])}")
    File.mkdir_p!(path)
    ExUnit.Callbacks.on_exit(fn -> File.rm_rf!(path) end)
    path
  end
end
