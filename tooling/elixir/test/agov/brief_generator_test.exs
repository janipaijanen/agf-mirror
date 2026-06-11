defmodule Agov.BriefGeneratorTest do
  use ExUnit.Case, async: true

  alias Agov.BriefGenerator

  @channel_info %{next_seq: 1, total: 0, seq_range: nil, active_claims: 0}
  @channel_info_21 %{next_seq: 22, total: 21, seq_range: {1, 21}, active_claims: 3}

  describe "generate/2 — default note kind" do
    test "contains seq from channel_info" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info_21)
      assert skeleton =~ "seq: 22"
    end

    test "contains kind: note by default" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info)
      assert skeleton =~ "kind: note"
    end

    test "contains placeholder tokens when fields not supplied" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info)
      assert skeleton =~ "<from>"
      assert skeleton =~ "<to>"
      assert skeleton =~ "<topic>"
      assert skeleton =~ "<TIMESTAMP>"
    end

    test "channel summary header present" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info_21)
      assert skeleton =~ "21 messages"
      assert skeleton =~ "seq 1"
      assert skeleton =~ "Next seq: 22"
    end

    test "empty channel shows 0 messages and seq 1" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info)
      assert skeleton =~ "0 messages"
      assert skeleton =~ "Next seq: 1"
    end
  end

  describe "generate/2 — from/to/topic fields" do
    test "from option replaces placeholder" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info, from: "planner-architect")
      assert skeleton =~ "from: planner-architect"
      refute skeleton =~ "<from>"
    end

    test "to option replaces placeholder" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info, to: "governor-reviewer")
      assert skeleton =~ "to: governor-reviewer"
      refute skeleton =~ "<to>"
    end

    test "topic option replaces placeholder" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info, topic: "T1-4-coord-brief")
      assert skeleton =~ "topic: T1-4-coord-brief"
      refute skeleton =~ "<topic>"
    end
  end

  describe "generate/2 — handoff kind" do
    test "contains handoff: block" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info, kind: "handoff")
      assert skeleton =~ "handoff:"
      assert skeleton =~ "next_agent:"
      assert skeleton =~ "status: ready"
      assert skeleton =~ "do_not_touch:"
    end

    test "next_agent uses to value when supplied" do
      {:ok, skeleton} =
        BriefGenerator.generate(@channel_info, kind: "handoff", to: "coder-tester")

      assert skeleton =~ "next_agent: coder-tester"
    end
  end

  describe "generate/2 — claim kind" do
    test "contains artifacts_owned with reminder comment" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info, kind: "claim")
      assert skeleton =~ "artifacts_owned:"
      assert skeleton =~ "<path>"
    end
  end

  describe "generate/2 — completion kind" do
    test "contains artifacts_owned release comment" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info, kind: "completion")
      assert skeleton =~ "artifacts_owned:"
      assert skeleton =~ "releases all"
    end
  end

  describe "generate/2 — ack kind" do
    test "contains references-to with reminder comment" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info, kind: "ack")
      assert skeleton =~ "references-to:"
      assert skeleton =~ "acknowledges"
    end
  end

  describe "generate/2 — review kind" do
    test "contains review body hint" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info, kind: "review")
      assert skeleton =~ "findings"
    end
  end

  describe "generate/2 — unknown kind" do
    test "returns error tuple" do
      assert {:error, {:unknown_kind, "bogus"}} =
               BriefGenerator.generate(@channel_info, kind: "bogus")
    end

    test "returns error for empty string kind" do
      assert {:error, {:unknown_kind, ""}} =
               BriefGenerator.generate(@channel_info, kind: "")
    end
  end

  describe "generate/2 — active claims in header" do
    test "shows active claims count when nonzero" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info_21)
      assert skeleton =~ "3 active"
    end

    test "shows no active claims when zero" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info)
      assert skeleton =~ "no active"
    end
  end
end
