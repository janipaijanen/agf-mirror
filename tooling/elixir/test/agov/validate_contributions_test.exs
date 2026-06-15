defmodule Agov.ValidateContributionsTest do
  use ExUnit.Case, async: true

  test "emits advisory warnings without failing" do
    dir = tmp_dir("contributions")
    path = Path.join(dir, "learning.yaml")

    File.write!(path, """
    project:
      codename: fixture
    learnings:
      - title: Quiet rollout
        description: No issues were observed.
        confidence:
          level: multi-adopter
          evidence_count: 1
    """)

    assert {:ok, warnings} = Agov.ValidateContributions.validate_all(dir)
    assert Enum.any?(warnings, &(&1.message == "absence-of-incident claim requires evidence"))
    assert Enum.any?(warnings, &String.contains?(&1.message, "weak evidence"))
  end

  defp tmp_dir(name) do
    path = Path.join(System.tmp_dir!(), "agov-#{name}-#{System.unique_integer([:positive])}")
    File.mkdir_p!(path)
    ExUnit.Callbacks.on_exit(fn -> File.rm_rf!(path) end)
    path
  end
end
