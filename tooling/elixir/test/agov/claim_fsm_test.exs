defmodule Agov.ClaimFSMTest do
  use ExUnit.Case, async: true

  alias Agov.ClaimFSM

  @now ~U[2026-01-01 00:00:00Z]
  @future ~U[2026-01-02 00:00:00Z]
  @past ~U[2025-12-31 00:00:00Z]

  defp entry(from, seq, expires_on) do
    %{from: from, seq: seq, claimed_at: "2026-01-01T00:00:00Z", expires_on: expires_on}
  end

  describe "claim transitions" do
    test "unclaimed claim becomes active when not expired" do
      event = {:claim, "coder-tester", 1, "2026-01-01T00:00:00Z", @future}
      assert {:active, entry} = ClaimFSM.transition(:unclaimed, event, @now)
      assert entry.from == "coder-tester"
      assert entry.seq == 1
    end

    test "unclaimed claim stores ULID id when present" do
      event = {:claim, "coder-tester", nil, "01KV6BPDNNDCBMEWER3ZFKQHVG", "2026-01-01T00:00:00Z", @future}
      assert {:active, entry} = ClaimFSM.transition(:unclaimed, event, @now)
      assert entry.id == "01KV6BPDNNDCBMEWER3ZFKQHVG"
      assert entry.seq == nil
    end

    test "unclaimed claim becomes stale when already expired" do
      event = {:claim, "coder-tester", 1, "2026-01-01T00:00:00Z", @past}
      assert {:stale, %{seq: 1}} = ClaimFSM.transition(:unclaimed, event, @now)
    end

    test "active claim is superseded by a later active claim" do
      old = {:active, entry("planner-architect", 1, @future)}
      event = {:claim, "coder-tester", 2, "2026-01-01T00:00:00Z", nil}
      assert {:active, %{from: "coder-tester", seq: 2}} = ClaimFSM.transition(old, event, @now)
    end

    test "active claim is superseded by a later stale claim" do
      old = {:active, entry("planner-architect", 1, @future)}
      event = {:claim, "coder-tester", 2, "2026-01-01T00:00:00Z", @past}
      assert {:stale, %{from: "coder-tester", seq: 2}} = ClaimFSM.transition(old, event, @now)
    end

    test "stale claim is superseded by a later active claim" do
      old = {:stale, entry("planner-architect", 1, @past)}
      event = {:claim, "coder-tester", 2, "2026-01-01T00:00:00Z", @future}
      assert {:active, %{from: "coder-tester", seq: 2}} = ClaimFSM.transition(old, event, @now)
    end

    test "stale claim is superseded by a later stale claim" do
      old = {:stale, entry("planner-architect", 1, @past)}
      event = {:claim, "coder-tester", 2, "2026-01-01T00:00:00Z", @past}
      assert {:stale, %{from: "coder-tester", seq: 2}} = ClaimFSM.transition(old, event, @now)
    end
  end

  describe "completion transitions" do
    test "active claim completed by owner becomes released" do
      state = {:active, entry("coder-tester", 1, @future)}

      assert {:released, %{seq: 1}} =
               ClaimFSM.transition(state, {:complete, "coder-tester"}, @now)
    end

    test "stale claim completed by owner becomes released" do
      state = {:stale, entry("coder-tester", 1, @past)}

      assert {:released, %{seq: 1}} =
               ClaimFSM.transition(state, {:complete, "coder-tester"}, @now)
    end

    test "active claim completed by another role stays active" do
      state = {:active, entry("coder-tester", 1, @future)}
      assert ^state = ClaimFSM.transition(state, {:complete, "planner-architect"}, @now)
    end

    test "stale claim completed by another role stays stale" do
      state = {:stale, entry("coder-tester", 1, @past)}
      assert ^state = ClaimFSM.transition(state, {:complete, "planner-architect"}, @now)
    end

    test "unclaimed completion is a no-op" do
      assert :unclaimed = ClaimFSM.transition(:unclaimed, {:complete, "coder-tester"}, @now)
    end
  end

  describe "tick transitions" do
    test "active claim without expiry stays active" do
      state = {:active, entry("coder-tester", 1, nil)}
      assert ^state = ClaimFSM.transition(state, {:tick, @now}, @now)
    end

    test "active claim with future expiry stays active" do
      state = {:active, entry("coder-tester", 1, @future)}
      assert ^state = ClaimFSM.transition(state, {:tick, @now}, @now)
    end

    test "active claim past expiry becomes stale" do
      state = {:active, entry("coder-tester", 1, @past)}
      assert {:stale, %{seq: 1}} = ClaimFSM.transition(state, {:tick, @now}, @now)
    end

    test "stale claim remains stale on tick" do
      state = {:stale, entry("coder-tester", 1, @past)}
      assert ^state = ClaimFSM.transition(state, {:tick, @now}, @now)
    end
  end

  describe "released terminal state" do
    test "released ignores claim events" do
      state = {:released, entry("coder-tester", 1, nil)}
      event = {:claim, "planner-architect", 2, "2026-01-01T00:00:00Z", @future}
      assert ^state = ClaimFSM.transition(state, event, @now)
    end

    test "released ignores completion events" do
      state = {:released, entry("coder-tester", 1, nil)}
      assert ^state = ClaimFSM.transition(state, {:complete, "coder-tester"}, @now)
    end

    test "released ignores tick events" do
      state = {:released, entry("coder-tester", 1, nil)}
      assert ^state = ClaimFSM.transition(state, {:tick, @future}, @now)
    end
  end
end
