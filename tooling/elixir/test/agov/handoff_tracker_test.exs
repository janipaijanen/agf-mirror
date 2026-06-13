defmodule Agov.HandoffTrackerTest do
  use ExUnit.Case, async: true

  alias Agov.HandoffTracker

  defp handoff(filename, seq, to, topic \\ "test-topic") do
    {filename,
     %{
       "kind" => "handoff",
       "from" => "planner-architect",
       "to" => to,
       "seq" => seq,
       "topic" => topic,
       "handoff" => %{"status" => "ready"}
     }}
  end

  defp msg_referencing(seq, ref_filenames) do
    {"#{String.pad_leading(to_string(seq), 5, "0")}-responder.md",
     %{
       "kind" => "decision-record",
       "from" => "governor-reviewer",
       "to" => "coder-tester",
       "seq" => seq,
       "references-to" => ref_filenames
     }}
  end

  defp handoff_mentioning(filename, seq, to, mentioned_filename) do
    {filename,
     %{
       "kind" => "handoff",
       "from" => "coder-tester",
       "to" => to,
       "seq" => seq,
       "topic" => "bridge-topic",
       "references-to" => [],
       "handoff" => %{"status" => "ready"},
       "_body" => "Bridge keeps #{mentioned_filename} visible."
     }}
  end

  describe "scan/1 — resolution by references-to" do
    test "handoff referenced by a later message is resolved" do
      messages = [
        handoff("00010-planner-architect.md", 10, "governor-reviewer"),
        msg_referencing(11, ["00010-planner-architect.md"])
      ]

      {:ok, result} = HandoffTracker.scan(messages)
      assert result.pending == []
      assert [entry] = result.resolved
      assert entry.seq == 10
    end

    test "handoff with no later reference is pending" do
      messages = [
        handoff("00010-planner-architect.md", 10, "governor-reviewer")
      ]

      {:ok, result} = HandoffTracker.scan(messages)
      assert [entry] = result.pending
      assert entry.seq == 10
      assert result.resolved == []
    end

    test "two handoffs: one referenced, one not" do
      messages = [
        handoff("00010-planner-architect.md", 10, "governor-reviewer", "topic-a"),
        handoff("00011-planner-architect.md", 11, "coder-tester", "topic-b"),
        msg_referencing(12, ["00010-planner-architect.md"])
      ]

      {:ok, result} = HandoffTracker.scan(messages)
      assert length(result.resolved) == 1
      assert hd(result.resolved).seq == 10
      assert length(result.pending) == 1
      assert hd(result.pending).seq == 11
    end

    test "reference at lower seq than the handoff does not resolve it" do
      messages = [
        msg_referencing(5, ["00010-planner-architect.md"]),
        handoff("00010-planner-architect.md", 10, "governor-reviewer")
      ]

      {:ok, result} = HandoffTracker.scan(messages)
      assert [_] = result.pending
      assert result.resolved == []
    end

    test "full path references resolve by basename" do
      messages = [
        handoff("00010-planner-architect.md", 10, "governor-reviewer"),
        msg_referencing(11, ["docs/coordination/00010-planner-architect.md"])
      ]

      {:ok, result} = HandoffTracker.scan(messages)
      assert result.pending == []
      assert [%{seq: 10}] = result.resolved
    end

    test "body mention alone does not resolve a handoff" do
      messages = [
        handoff("00010-coder-tester.md", 10, "governor-reviewer"),
        handoff_mentioning("00011-coder-tester.md", 11, "governor-reviewer", "00010-coder-tester.md")
      ]

      {:ok, result} = HandoffTracker.scan(messages)
      assert Enum.map(result.pending, & &1.seq) == [11, 10]
      assert result.resolved == []
    end

    test "handoff resolves through body mention and later decision chain" do
      messages = [
        handoff("00189-coder-tester.md", 189, "governor-reviewer", "cold-start docs"),
        handoff_mentioning(
          "00190-coder-tester.md",
          190,
          "governor-reviewer",
          "docs/coordination/00189-coder-tester.md"
        ),
        {"00192-planner-architect.md",
         %{
           "kind" => "review",
           "from" => "planner-architect",
           "to" => "governor-reviewer",
           "seq" => 192,
           "topic" => "gate2-review",
           "references-to" => ["00190-coder-tester.md"]
         }},
        msg_referencing(210, ["00192-planner-architect.md"])
      ]

      {:ok, result} = HandoffTracker.scan(messages)
      assert result.pending == []
      assert Enum.map(result.resolved, & &1.seq) == [190, 189]
    end

    test "non-handoff messages are not classified" do
      messages = [
        {"00010-coder-tester.md",
         %{"kind" => "completion", "from" => "coder-tester", "seq" => 10,
           "references-to" => [], "artifacts_owned" => []}}
      ]

      {:ok, result} = HandoffTracker.scan(messages)
      assert result.pending == []
      assert result.resolved == []
    end

    test "empty message list returns empty result" do
      assert {:ok, %{pending: [], resolved: []}} = HandoffTracker.scan([])
    end
  end

  describe "scan/1 — resolved entry fields" do
    test "resolved entry contains seq, from, to, topic, status" do
      messages = [
        handoff("00010-planner-architect.md", 10, "governor-reviewer", "my-topic"),
        msg_referencing(11, ["00010-planner-architect.md"])
      ]

      {:ok, %{resolved: [entry]}} = HandoffTracker.scan(messages)
      assert entry.seq == 10
      assert entry.from == "planner-architect"
      assert entry.to == "governor-reviewer"
      assert entry.topic == "my-topic"
      assert entry.status == "ready"
    end

    test "pending entry contains same fields" do
      messages = [handoff("00010-planner-architect.md", 10, "governor-reviewer", "my-topic")]
      {:ok, %{pending: [entry]}} = HandoffTracker.scan(messages)
      assert entry.seq == 10
      assert entry.from == "planner-architect"
      assert entry.to == "governor-reviewer"
      assert entry.topic == "my-topic"
    end
  end

  describe "scan/1 — ordering" do
    test "pending handoffs are sorted by seq descending" do
      messages = [
        handoff("00005-planner-architect.md", 5, "governor-reviewer"),
        handoff("00003-planner-architect.md", 3, "governor-reviewer")
      ]

      {:ok, %{pending: entries}} = HandoffTracker.scan(messages)
      seqs = Enum.map(entries, & &1.seq)
      assert seqs == Enum.sort(seqs, :desc)
    end
  end
end
