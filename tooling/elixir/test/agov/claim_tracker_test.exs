defmodule Agov.ClaimTrackerTest do
  use ExUnit.Case, async: true

  alias Agov.{ClaimTracker, CoordValidator}

  @fixtures_dir Path.join([__DIR__, "..", "fixtures"])
  @fixed_now ~U[2026-01-01 00:00:00Z]

  defp load(filename) do
    {:ok, data} = CoordValidator.validate_file(Path.join(@fixtures_dir, filename))
    data
  end

  defp completion_from(agent) do
    %{"kind" => "completion", "from" => agent, "artifacts_owned" => []}
  end

  describe "scan/2 — stale and active classification" do
    test "expired claim is stale" do
      {:ok, result} = ClaimTracker.scan([load("stale_claim.md")], now: @fixed_now)
      assert [entry] = result.stale
      assert entry.path == "docs/stale.md"
      assert entry.from == "planner-architect"
      assert entry.seq == 100
      assert result.active == []
      assert result.released == []
    end

    test "non-expired claim is active" do
      {:ok, result} = ClaimTracker.scan([load("active_claim.md")], now: @fixed_now)
      assert [entry] = result.active
      assert entry.path == "docs/active.md"
      assert result.stale == []
    end

    test "claim without expires_on is always active, never stale" do
      {:ok, result} = ClaimTracker.scan([load("claim_no_expiry.md")], now: @fixed_now)
      assert [entry] = result.active
      assert entry.path == "docs/noexpiry.md"
      assert entry.expires_on == nil
      assert result.stale == []
    end
  end

  describe "scan/2 — release semantics" do
    test "completion from same agent releases claim" do
      messages = [load("claim_no_expiry.md"), completion_from("coder-tester")]
      {:ok, result} = ClaimTracker.scan(messages, now: @fixed_now)
      assert [entry] = result.released
      assert entry.path == "docs/noexpiry.md"
      assert result.active == []
      assert result.stale == []
    end

    test "completion from different agent does not release claim" do
      messages = [load("claim_no_expiry.md"), completion_from("planner-architect")]
      {:ok, result} = ClaimTracker.scan(messages, now: @fixed_now)
      assert [_] = result.active
      assert result.released == []
    end

    test "completion does not affect other agents' claims" do
      stale = load("stale_claim.md")
      no_expiry = load("claim_no_expiry.md")
      # coder-tester completes; should release coder-tester's claim only
      messages = [stale, no_expiry, completion_from("coder-tester")]
      {:ok, result} = ClaimTracker.scan(messages, now: @fixed_now)
      assert [_] = result.stale
      assert [_] = result.released
      assert result.active == []
    end
  end

  describe "scan/2 — supersession" do
    test "later claim for same path supersedes earlier (same agent)" do
      old_claim = load("stale_claim.md")
      # seq 200 supersedes seq 100; no expires_on so never stale
      new_claim = %{
        "kind" => "claim",
        "from" => "planner-architect",
        "seq" => 200,
        "timestamp" => "2025-06-01T00:00:00Z",
        "artifacts_owned" => ["docs/stale.md"]
      }
      {:ok, result} = ClaimTracker.scan([old_claim, new_claim], now: @fixed_now)
      assert [entry] = result.active
      assert entry.seq == 200
      assert result.stale == []
    end

    test "later claim for same path supersedes earlier (different agent)" do
      old_claim = load("stale_claim.md")
      new_claim = %{
        "kind" => "claim",
        "from" => "coder-tester",
        "seq" => 200,
        "artifacts_owned" => ["docs/stale.md"]
      }
      {:ok, result} = ClaimTracker.scan([old_claim, new_claim], now: @fixed_now)
      assert [entry] = result.active
      assert entry.from == "coder-tester"
      assert result.stale == []
    end
  end

  describe "scan/2 — partial release (non-empty artifacts_owned on completion)" do
    defp claim(from, path, seq \\ 50) do
      %{
        "kind" => "claim",
        "from" => from,
        "seq" => seq,
        "timestamp" => "2026-01-01T00:00:00Z",
        "artifacts_owned" => [path]
      }
    end

    defp partial_completion(from, paths) do
      %{"kind" => "completion", "from" => from, "artifacts_owned" => paths}
    end

    test "partial completion releases only the listed paths" do
      messages = [
        claim("coder-tester", "docs/a.md", 10),
        claim("coder-tester", "docs/b.md", 11),
        partial_completion("coder-tester", ["docs/a.md"])
      ]

      {:ok, result} = ClaimTracker.scan(messages, now: @fixed_now)
      assert length(result.released) == 1
      assert hd(result.released).path == "docs/a.md"
      assert length(result.active) == 1
      assert hd(result.active).path == "docs/b.md"
    end

    test "partial completion with unlisted path leaves it active" do
      messages = [
        claim("coder-tester", "docs/c.md", 20),
        partial_completion("coder-tester", ["docs/other.md"])
      ]

      {:ok, result} = ClaimTracker.scan(messages, now: @fixed_now)
      assert [entry] = result.active
      assert entry.path == "docs/c.md"
      assert result.released == []
    end

    test "partial completion from different agent does nothing" do
      messages = [
        claim("coder-tester", "docs/d.md", 30),
        partial_completion("planner-architect", ["docs/d.md"])
      ]

      {:ok, result} = ClaimTracker.scan(messages, now: @fixed_now)
      assert [_] = result.active
      assert result.released == []
    end
  end

  describe "scan/2 — from_aliases option" do
    defp aliased_claim(from, path) do
      %{
        "kind" => "claim",
        "from" => from,
        "seq" => 60,
        "timestamp" => "2026-01-01T00:00:00Z",
        "artifacts_owned" => [path]
      }
    end

    @aliases %{"claude" => "planner-architect"}

    test "completion from canonical releases claim from alias" do
      messages = [
        aliased_claim("claude", "docs/old.md"),
        %{"kind" => "completion", "from" => "planner-architect", "artifacts_owned" => []}
      ]

      {:ok, result} = ClaimTracker.scan(messages, now: @fixed_now, from_aliases: @aliases)
      assert [entry] = result.released
      assert entry.path == "docs/old.md"
      assert result.active == []
    end

    test "completion from alias releases claim from canonical" do
      messages = [
        aliased_claim("planner-architect", "docs/new.md"),
        %{"kind" => "completion", "from" => "claude", "artifacts_owned" => []}
      ]

      {:ok, result} = ClaimTracker.scan(messages, now: @fixed_now, from_aliases: @aliases)
      assert [entry] = result.released
      assert entry.path == "docs/new.md"
      assert result.active == []
    end

    test "partial completion from canonical releases alias claim for listed paths" do
      messages = [
        aliased_claim("claude", "docs/x.md"),
        aliased_claim("claude", "docs/y.md"),
        %{"kind" => "completion", "from" => "planner-architect",
          "artifacts_owned" => ["docs/x.md"]}
      ]

      {:ok, result} = ClaimTracker.scan(messages, now: @fixed_now, from_aliases: @aliases)
      assert length(result.released) == 1
      assert hd(result.released).path == "docs/x.md"
      assert length(result.active) == 1
      assert hd(result.active).path == "docs/y.md"
    end

    test "without aliases, strict from matching unchanged" do
      messages = [
        aliased_claim("claude", "docs/old.md"),
        %{"kind" => "completion", "from" => "planner-architect", "artifacts_owned" => []}
      ]

      {:ok, result} = ClaimTracker.scan(messages, now: @fixed_now)
      assert [_] = result.active
      assert result.released == []
    end
  end

  describe "scan/2 — invalid expires_on" do
    test "invalid expires_on on a claim returns error with seq" do
      assert {:error, [{:invalid_expires_on, 103, "not-a-date"}]} =
               ClaimTracker.scan([load("invalid_expires_on.md")], now: @fixed_now)
    end

    test "invalid expires_on on non-claim message is ignored" do
      non_claim = %{
        "kind" => "note",
        "from" => "planner-architect",
        "expires_on" => "not-a-date"
      }
      assert {:ok, %{stale: [], active: [], released: []}} =
               ClaimTracker.scan([non_claim], now: @fixed_now)
    end
  end
end
