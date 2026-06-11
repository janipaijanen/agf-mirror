defmodule Agov.HandoffReaderTest do
  use ExUnit.Case, async: true

  alias Agov.HandoffReader

  @fixtures_dir Path.join([__DIR__, "..", "fixtures"])

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
      assert String.contains?(output, "spec/schemas/coordination-message.schema.json")
    end

    test "formats a blocked handoff with status prominently" do
      {:ok, data} = HandoffReader.find_next(@fixtures_dir, "governor-reviewer")
      output = HandoffReader.format_summary(data, "handoff_blocked.md")
      assert String.contains?(output, "blocked")
    end
  end
end
