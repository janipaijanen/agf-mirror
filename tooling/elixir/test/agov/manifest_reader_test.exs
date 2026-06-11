defmodule Agov.ManifestReaderTest do
  use ExUnit.Case, async: true

  alias Agov.ManifestReader

  @fixtures_dir Path.join([__DIR__, "..", "fixtures"])
  @manifest_path Path.join(@fixtures_dir, "test_manifest.yaml")

  setup do
    content = """
    agents:
      planner-architect:
        status: active
        played_by: "claude"
      coder-tester:
        status: active
        played_by: "claude"
      governor-reviewer:
        status: active
        played_by: "human"
      build-engineer:
        status: suspended
        reason: "no CI yet"
      documentation-writer:
        status: skipped
        reason: "not needed"
    """

    File.write!(@manifest_path, content)
    on_exit(fn -> File.rm(@manifest_path) end)
    :ok
  end

  describe "load/1 — valid manifest" do
    test "returns ok with agent map" do
      assert {:ok, agents} = ManifestReader.load(@manifest_path)
      assert is_map(agents)
    end

    test "active agent has status: active" do
      {:ok, agents} = ManifestReader.load(@manifest_path)
      assert agents["planner-architect"].status == "active"
    end

    test "suspended agent has status: suspended" do
      {:ok, agents} = ManifestReader.load(@manifest_path)
      assert agents["build-engineer"].status == "suspended"
    end

    test "skipped agent has status: skipped" do
      {:ok, agents} = ManifestReader.load(@manifest_path)
      assert agents["documentation-writer"].status == "skipped"
    end

    test "all five agents are present" do
      {:ok, agents} = ManifestReader.load(@manifest_path)
      assert map_size(agents) == 5
    end
  end

  describe "load/1 — missing file" do
    test "returns error :enoent for missing file" do
      assert {:error, :enoent} = ManifestReader.load("/nonexistent/path/MANIFEST.yaml")
    end
  end

  describe "load/1 — edge cases" do
    test "manifest with no agents section returns empty map" do
      path = Path.join(@fixtures_dir, "empty_manifest.yaml")
      File.write!(path, "other_key: value\n")
      on_exit(fn -> File.rm(path) end)
      assert {:ok, %{}} = ManifestReader.load(path)
    end

    test "ignores non-map agent entries" do
      path = Path.join(@fixtures_dir, "odd_manifest.yaml")
      File.write!(path, "agents:\n  good-agent:\n    status: active\n  bad-entry: not-a-map\n")
      on_exit(fn -> File.rm(path) end)
      {:ok, agents} = ManifestReader.load(path)
      assert Map.has_key?(agents, "good-agent")
      refute Map.has_key?(agents, "bad-entry")
    end
  end
end
