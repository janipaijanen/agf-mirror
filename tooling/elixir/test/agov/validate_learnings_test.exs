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

  test "accepts timestamp and historical learning ids" do
    dir = tmp_dir("learnings")
    path = Path.join(dir, "ids.yaml")

    File.write!(path, """
    project:
      codename: foundry-fixture
    learnings:
      - id: L021
        title: Historical id
        description: Generic finding.
      - id: 20260621T150000Z-a3fx
        title: Timestamp id with suffix
        description: Generic finding.
    """)

    assert {:ok, _data} = Agov.ValidateLearnings.validate_file(path)
  end

  test "skips template learning files in collection validation" do
    dir = tmp_dir("learnings")
    path = Path.join(dir, "_template.yaml")

    File.write!(path, """
    project:
      codename: ""
    learnings: []
    """)

    assert {:ok, []} = Agov.ValidateLearnings.validate_all(dir)
  end

  test "warns for bare timestamp learning ids" do
    dir = tmp_dir("learnings")
    path = Path.join(dir, "bare-timestamp.yaml")

    File.write!(path, """
    project:
      codename: foundry-fixture
    learnings:
      - id: 20260621T150000Z
        title: Bare timestamp id
        description: Generic finding.
    """)

    assert {:error, errors} = Agov.ValidateLearnings.validate_file(path)

    assert Enum.any?(errors, fn error ->
             error.message =~ "bare timestamp learning id" and error.severity == :advisory
           end)

    assert Enum.all?(errors, &(&1.severity == :advisory))
  end

  test "rejects non-canonical learning ids" do
    dir = tmp_dir("learnings")
    path = Path.join(dir, "bad-id.yaml")

    File.write!(path, """
    project:
      codename: foundry-fixture
    learnings:
      - id: learn-20260621T145130Z-os-background-timers-7rhn
        title: Bad id
        description: Generic finding.
    """)

    assert {:error, errors} = Agov.ValidateLearnings.validate_file(path)
    assert Enum.any?(errors, &String.contains?(&1.message, "learning id"))
  end

  test "rejects duplicate learning ids" do
    dir = tmp_dir("learnings")
    path = Path.join(dir, "duplicate-id.yaml")

    File.write!(path, """
    project:
      codename: foundry-fixture
    learnings:
      - id: 20260621T150000Z
        title: First
        description: Generic finding.
      - id: 20260621T150000Z
        title: Second
        description: Generic finding.
    """)

    assert {:error, errors} = Agov.ValidateLearnings.validate_file(path)
    assert Enum.any?(errors, &String.contains?(&1.message, "duplicate learning id"))
  end

  defp tmp_dir(name) do
    path = Path.join(System.tmp_dir!(), "agov-#{name}-#{System.unique_integer([:positive])}")
    File.mkdir_p!(path)
    ExUnit.Callbacks.on_exit(fn -> File.rm_rf!(path) end)
    path
  end
end
