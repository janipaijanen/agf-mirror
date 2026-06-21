defmodule Agov.BriefGeneratorTest do
  use ExUnit.Case, async: true

  alias Agov.BriefGenerator
  alias Agov.CoordBrief

  @next_id "01KV6BPDNNDCBMEWER3ZFKQHVG"
  @channel_info %{next_id: @next_id, total: 0, timestamp_range: nil, active_claims: 0}
  @channel_info_21 %{
    next_id: @next_id,
    total: 21,
    timestamp_range: {"2026-06-13T00:00:00Z", "2026-06-15T00:00:00Z"},
    active_claims: 3
  }

  describe "generate/2 — default note kind" do
    test "contains id from channel_info" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info_21)
      assert skeleton =~ "id: #{@next_id}"
      refute skeleton =~ "seq:"
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
      assert skeleton =~ "2026-06-13T00:00:00Z to 2026-06-15T00:00:00Z"
      assert skeleton =~ "Next id: #{@next_id}"
      assert skeleton =~ "Suggested file: #{@next_id}.md"
    end

    test "empty channel shows 0 messages and next id" do
      {:ok, skeleton} = BriefGenerator.generate(@channel_info)
      assert skeleton =~ "0 messages"
      assert skeleton =~ "Next id: #{@next_id}"
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

  describe "CoordBrief.generate/1 — id format" do
    test "generates readable id skeleton when explicitly requested" do
      {:ok, skeleton} =
        CoordBrief.generate([
          "--dir",
          Path.join([__DIR__, "..", "fixtures"]),
          "--id-format",
          "readable",
          "--from",
          "coder-tester"
        ])

      assert skeleton =~ ~r/id: [0-9]{8}T[0-9]{6}Z-coder-tester-[a-z0-9]{4}/
      assert skeleton =~ "from: coder-tester"
      assert skeleton =~ ~r/timestamp: [0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z/
      assert skeleton =~ ~r/Suggested file: [0-9]{8}T[0-9]{6}Z-coder-tester-[a-z0-9]{4}\.md/
      refute skeleton =~ ~r/Suggested file: [0-9]{8}T[0-9]{6}Z-coder-tester-[a-z0-9]{4}-coder-tester\.md/
      refute skeleton =~ "<TIMESTAMP>"
    end

    test "readable id format requires a sender role" do
      assert {:error, :readable_requires_from} = CoordBrief.generate(["--id-format", "readable"])
    end
  end
end
