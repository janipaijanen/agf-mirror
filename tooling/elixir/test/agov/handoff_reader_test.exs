defmodule Agov.HandoffReaderTest do
  use ExUnit.Case, async: true

  alias Agov.HandoffReader

  @fixtures_dir Path.join([__DIR__, "..", "fixtures"])

  defp tmp_fixture_dir!(test) do
    dir = Path.join(System.tmp_dir!(), "agov-handoff-reader-#{test}-#{System.unique_integer([:positive])}")
    File.rm_rf!(dir)
    File.mkdir_p!(dir)
    on_exit(fn -> File.rm_rf!(dir) end)
    dir
  end

  defp copy_fixture!(dir, filename) do
    File.cp!(Path.join(@fixtures_dir, filename), Path.join(dir, filename))
  end

  defp write_decision!(dir, filename, seq, references) do
    refs = Enum.map_join(references, "\n", &"  - #{&1}")

    File.write!(Path.join(dir, filename), """
    ---
    kind: decision-record
    from: governor-reviewer
    to: planner-architect
    seq: #{seq}
    timestamp: 2026-06-11T00:02:00Z
    immutable: true
    references-to:
    #{refs}
    topic: handoff-resolution-fixture
    requires-ack: false
    artifacts_owned: []
    ---

    Decision record resolving a handoff fixture.
    """)
  end

  describe "find_next/2" do
    test "returns the highest-seq handoff addressed to the given role" do
      # fixtures dir has handoff_ready.md (seq 50, to: coder-tester)
      # and handoff_older.md (seq 45, to: coder-tester)
      assert {:ok, data} = HandoffReader.find_next(@fixtures_dir, "coder-tester")
      assert data["seq"] == 50
      assert data["handoff"]["status"] == "ready"
    end

    test "returns {:none} when no handoff exists for the role" do
      assert {:none} = HandoffReader.find_next(@fixtures_dir, "nonexistent-role")
    end

    test "skips a handoff resolved by a later referencing message" do
      dir = tmp_fixture_dir!("resolved-latest")
      copy_fixture!(dir, "handoff_ready.md")
      copy_fixture!(dir, "handoff_older.md")
      write_decision!(dir, "00052-governor-reviewer.md", 52, ["handoff_ready.md"])

      assert {:ok, data} = HandoffReader.find_next(dir, "coder-tester")
      assert data["seq"] == 45
      assert data["_source_file"] == "handoff_older.md"
    end

    test "returns {:none} when all handoffs for the role are resolved" do
      dir = tmp_fixture_dir!("resolved-all")
      copy_fixture!(dir, "handoff_ready.md")
      write_decision!(dir, "00052-governor-reviewer.md", 52, ["handoff_ready.md"])

      assert {:none} = HandoffReader.find_next(dir, "coder-tester")
    end

    test "returns blocked handoff for governor-reviewer" do
      assert {:ok, data} = HandoffReader.find_next(@fixtures_dir, "governor-reviewer")
      assert data["handoff"]["status"] == "blocked"
    end

    test "returns {:error, :dir_not_found} for missing directory" do
      assert {:error, :dir_not_found} = HandoffReader.find_next("/nonexistent/dir", "coder-tester")
    end
  end

  describe "format_summary/2" do
    test "formats a ready handoff with all fields" do
      {:ok, data} = HandoffReader.find_next(@fixtures_dir, "coder-tester")
      output = HandoffReader.format_summary(data, "handoff_ready.md")
      assert String.contains?(output, "coder-tester")
      assert String.contains?(output, "ready")
      assert String.contains?(output, "Implement HandoffReader module")
      assert String.contains?(output, "docs/plans/T1-3-handoff-state-tooling.md")
      assert String.contains?(output, "agf/schemas/coordination-message.schema.json")
    end

    test "formats a blocked handoff with status prominently" do
      {:ok, data} = HandoffReader.find_next(@fixtures_dir, "governor-reviewer")
      output = HandoffReader.format_summary(data, "handoff_blocked.md")
      assert String.contains?(output, "blocked")
    end
  end
end
