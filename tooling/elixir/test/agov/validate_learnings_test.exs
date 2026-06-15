defmodule Agov.ValidateLearningsTest do
  use ExUnit.Case, async: true

  @fixtures_dir Path.join([__DIR__, "..", "fixtures"])

  test "accepts a valid learning record" do
    assert {:ok, _data} =
             Agov.ValidateLearnings.validate_file(Path.join(@fixtures_dir, "valid_learning.yaml"))
  end

  test "rejects freetext PII" do
    assert {:error, errors} =
             Agov.ValidateLearnings.validate_file(
               Path.join(@fixtures_dir, "invalid_learning_pii.yaml")
             )

    assert Enum.any?(errors, &(&1.message == "contains email address"))
  end

  test "requires corroboration for multi-adopter confidence" do
    dir = tmp_dir("learnings")
    path = Path.join(dir, "weak.yaml")

    File.write!(path, """
    project:
      codename: foundry-fixture
    learnings:
      - title: Weak confidence
        description: Generic finding.
        confidence:
          level: multi-adopter
          evidence_count: 1
    """)

    assert {:error, errors} = Agov.ValidateLearnings.validate_file(path)
    assert Enum.any?(errors, &String.contains?(&1.message, "requires evidence_count >= 2"))
  end

  defp tmp_dir(name) do
    path = Path.join(System.tmp_dir!(), "agov-#{name}-#{System.unique_integer([:positive])}")
    File.mkdir_p!(path)
    ExUnit.Callbacks.on_exit(fn -> File.rm_rf!(path) end)
    path
  end
end
