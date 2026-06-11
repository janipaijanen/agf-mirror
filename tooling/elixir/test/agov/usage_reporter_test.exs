defmodule Agov.UsageReporterTest do
  use ExUnit.Case, async: true

  alias Agov.UsageReporter

  defp msg(seq, filename, from, kind) do
    {filename, %{"seq" => seq, "from" => from, "kind" => kind}}
  end

  defp agents(list) do
    Map.new(list, fn {name, status} -> {name, %{status: status}} end)
  end

  describe "report/4 — kind and sender counts" do
    test "counts kinds correctly" do
      messages = [
        msg(1, "00001-a.md", "agent-a", "handoff"),
        msg(2, "00002-b.md", "agent-b", "handoff"),
        msg(3, "00003-a.md", "agent-a", "completion")
      ]

      result = UsageReporter.report(messages, %{})
      assert result.kind_counts["handoff"] == 2
      assert result.kind_counts["completion"] == 1
    end

    test "counts senders correctly" do
      messages = [
        msg(1, "00001-a.md", "agent-a", "handoff"),
        msg(2, "00002-a.md", "agent-a", "completion"),
        msg(3, "00003-b.md", "agent-b", "handoff")
      ]

      result = UsageReporter.report(messages, %{})
      assert result.sender_counts["agent-a"] == 2
      assert result.sender_counts["agent-b"] == 1
    end

    test "total reflects full channel, not just window" do
      messages = [
        msg(1, "00001-a.md", "agent-a", "note"),
        msg(2, "00002-a.md", "agent-a", "note"),
        msg(3, "00003-a.md", "agent-a", "note")
      ]

      result = UsageReporter.report(messages, %{}, %{}, last: 2)
      assert result.total == 3
      assert result.window == {:last, 2}
    end
  end

  describe "report/4 — inactive_agents" do
    test "active agent with no messages in window appears in inactive_agents" do
      messages = [msg(1, "00001-a.md", "agent-a", "note")]
      result = UsageReporter.report(messages, agents([{"agent-a", "active"}, {"agent-b", "active"}]))
      assert "agent-b" in result.inactive_agents
      refute "agent-a" in result.inactive_agents
    end

    test "suspended agent with no messages is NOT in inactive_agents" do
      messages = [msg(1, "00001-a.md", "agent-a", "note")]
      result = UsageReporter.report(messages, agents([{"agent-a", "active"}, {"agent-b", "suspended"}]))
      refute "agent-b" in result.inactive_agents
    end

    test "skipped agent with no messages is NOT in inactive_agents" do
      messages = [msg(1, "00001-a.md", "agent-a", "note")]
      result = UsageReporter.report(messages, agents([{"agent-a", "active"}, {"agent-b", "skipped"}]))
      refute "agent-b" in result.inactive_agents
    end

    test "empty message list puts all active agents in inactive_agents" do
      result = UsageReporter.report([], agents([{"agent-a", "active"}, {"agent-b", "active"}]))
      assert Enum.sort(result.inactive_agents) == ["agent-a", "agent-b"]
    end
  end

  describe "report/4 — alias resolution" do
    test "aliased sender counts under canonical name" do
      messages = [msg(1, "00001-old.md", "old-name", "note")]
      aliases = %{"old-name" => "new-name"}
      result = UsageReporter.report(messages, agents([{"new-name", "active"}]), aliases)
      assert result.sender_counts["new-name"] == 1
      refute Map.has_key?(result.sender_counts, "old-name")
    end

    test "aliased sender does not appear in unknown_senders" do
      messages = [msg(1, "00001-old.md", "old-name", "note")]
      aliases = %{"old-name" => "new-name"}
      result = UsageReporter.report(messages, agents([{"new-name", "active"}]), aliases)
      refute "new-name" in result.unknown_senders
      refute "old-name" in result.unknown_senders
    end

    test "aliased sender counts toward active agent; not in inactive_agents" do
      messages = [msg(1, "00001-old.md", "old-name", "note")]
      aliases = %{"old-name" => "canonical"}
      result = UsageReporter.report(messages, agents([{"canonical", "active"}]), aliases)
      refute "canonical" in result.inactive_agents
    end
  end

  describe "report/4 — unknown_senders" do
    test "sender not in MANIFEST (after alias) appears in unknown_senders" do
      messages = [msg(1, "00001-x.md", "mystery", "note")]
      result = UsageReporter.report(messages, agents([{"agent-a", "active"}]))
      assert "mystery" in result.unknown_senders
    end

    test "sender in MANIFEST does not appear in unknown_senders" do
      messages = [msg(1, "00001-a.md", "agent-a", "note")]
      result = UsageReporter.report(messages, agents([{"agent-a", "active"}]))
      refute "agent-a" in result.unknown_senders
    end

    test "empty MANIFEST puts all senders in unknown_senders" do
      messages = [msg(1, "00001-a.md", "agent-a", "note")]
      result = UsageReporter.report(messages, %{})
      assert "agent-a" in result.unknown_senders
    end
  end

  describe "report/4 — windowing with --last N" do
    test "last: N narrows sender counts to last N messages" do
      messages = [
        msg(1, "00001-a.md", "agent-a", "note"),
        msg(2, "00002-a.md", "agent-a", "note"),
        msg(3, "00003-b.md", "agent-b", "note")
      ]

      result = UsageReporter.report(messages, %{}, %{}, last: 1)
      assert result.sender_counts["agent-b"] == 1
      refute Map.has_key?(result.sender_counts, "agent-a")
    end

    test "window: :all when no last option given" do
      messages = [msg(1, "00001-a.md", "agent-a", "note")]
      result = UsageReporter.report(messages, %{})
      assert result.window == :all
    end

    test "ordering is seq ascending then filename ascending for tie-breaking" do
      messages = [
        msg(5, "00005-b.md", "agent-b", "note"),
        msg(5, "00005-a.md", "agent-a", "note"),
        msg(6, "00006-a.md", "agent-a", "note")
      ]

      result = UsageReporter.report(messages, %{}, %{}, last: 1)
      assert result.sender_counts["agent-a"] == 1
      refute Map.has_key?(result.sender_counts, "agent-b")
    end
  end

  describe "report/4 — invalid_files passthrough" do
    test "invalid_files are included in report" do
      result = UsageReporter.report([], %{}, %{}, invalid_files: ["bad-file.md"])
      assert result.invalid_files == ["bad-file.md"]
    end

    test "invalid_files defaults to empty list" do
      result = UsageReporter.report([], %{})
      assert result.invalid_files == []
    end
  end
end
