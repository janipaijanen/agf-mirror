defmodule Agov.TokenRoiTest do
  use ExUnit.Case, async: false

  alias Agov.TokenRoi

  setup do
    previous_enabled = System.get_env("AGOV_TOKEN_ROI")
    previous_log = System.get_env("AGOV_TOKEN_ROI_LOG")

    on_exit(fn ->
      restore_env("AGOV_TOKEN_ROI", previous_enabled)
      restore_env("AGOV_TOKEN_ROI_LOG", previous_log)
    end)

    :ok
  end

  test "estimate_tokens uses the documented chars divided by four approximation" do
    assert TokenRoi.estimate_tokens(0) == 0
    assert TokenRoi.estimate_tokens(1) == 1
    assert TokenRoi.estimate_tokens(4) == 1
    assert TokenRoi.estimate_tokens(5) == 2
  end

  test "record_tool_output appends JSONL records with char and token counts" do
    path = tmp_log_path()

    assert :ok =
             TokenRoi.record_tool_output(
               "agov.session.resume",
               ["--base-branch", "master"],
               "abcd",
               workflow: "cold-start",
               log_path: path
             )

    assert {:ok, [record]} = TokenRoi.load(path)
    assert record["event"] == "tool_output"
    assert record["command"] == "agov.session.resume"
    assert record["args"] == ["--base-branch", "master"]
    assert record["workflow"] == "cold-start"
    assert record["chars"] == 4
    assert record["approx_tokens"] == 1
    assert record["manual_baseline_min_tokens"] == 3_000
    assert record["manual_baseline_max_tokens"] == 5_000
    assert record["prompt_overhead_tokens"] > 0
    assert record["approx_saved_min_tokens"] > 0
    assert record["approx_saved_max_tokens"] > record["approx_saved_min_tokens"]
    assert is_binary(record["timestamp"])
  end

  test "workflow_baseline returns documented ranges for known live workflows" do
    assert {:ok, %{min_tokens: 3_000, max_tokens: 5_000}} =
             TokenRoi.workflow_baseline("cold-start")

    assert {:ok, %{min_tokens: 3_000, max_tokens: 5_000}} =
             TokenRoi.workflow_baseline("session-resume:coder-tester")

    assert {:ok, %{min_tokens: 1_000, max_tokens: 3_000}} =
             TokenRoi.workflow_baseline("next-handoff:governor-reviewer")

    assert :unknown = TokenRoi.workflow_baseline("custom")
  end

  test "record_tool_output honors AGOV_TOKEN_ROI disable switch" do
    path = tmp_log_path()
    System.put_env("AGOV_TOKEN_ROI", "0")

    assert :ok = TokenRoi.record_tool_output("agov.session.resume", [], "abcd", log_path: path)
    assert {:ok, []} = TokenRoi.load(path)
  end

  test "record_measurement can include a manual baseline for savings estimates" do
    path = tmp_log_path()

    assert :ok =
             TokenRoi.record_measurement(
               %{command: "manual", workflow: "cold-start", chars: 100, manual_chars: 500},
               log_path: path
             )

    assert {:ok, records} = TokenRoi.load(path)
    summary = TokenRoi.summarize(records)

    assert summary.total_records == 1
    assert summary.total_chars == 100
    assert summary.total_approx_tokens == 25
    assert summary.manual_chars == 500
    assert summary.manual_approx_tokens == 125
    assert summary.estimated_saved_min_tokens == 100
    assert summary.estimated_saved_max_tokens == 100

    assert %{
             command: "manual",
             count: 1,
             chars: 100,
             approx_tokens: 25,
             approx_saved_min_tokens: 100,
             approx_saved_max_tokens: 100
           } in summary.commands
  end

  test "summarize groups automatic tool output by command and workflow" do
    records = [
      %{
        "event" => "tool_output",
        "command" => "a",
        "workflow" => "x",
        "chars" => 4,
        "approx_tokens" => 1,
        "approx_saved_min_tokens" => 10,
        "approx_saved_max_tokens" => 20
      },
      %{
        "event" => "tool_output",
        "command" => "a",
        "workflow" => "x",
        "chars" => 8,
        "approx_tokens" => 2,
        "approx_saved_min_tokens" => 30,
        "approx_saved_max_tokens" => 40
      },
      %{
        "event" => "tool_output",
        "command" => "b",
        "workflow" => "y",
        "chars" => 12,
        "approx_tokens" => 3,
        "approx_saved_min_tokens" => 50,
        "approx_saved_max_tokens" => 60
      }
    ]

    summary = TokenRoi.summarize(records)

    assert %{
             command: "a",
             count: 2,
             chars: 12,
             approx_tokens: 3,
             approx_saved_min_tokens: 40,
             approx_saved_max_tokens: 60
           } in summary.commands

    assert %{
             command: "b",
             count: 1,
             chars: 12,
             approx_tokens: 3,
             approx_saved_min_tokens: 50,
             approx_saved_max_tokens: 60
           } in summary.commands

    assert %{
             workflow: "x",
             count: 2,
             chars: 12,
             approx_tokens: 3,
             approx_saved_min_tokens: 40,
             approx_saved_max_tokens: 60
           } in summary.workflows

    assert %{
             workflow: "y",
             count: 1,
             chars: 12,
             approx_tokens: 3,
             approx_saved_min_tokens: 50,
             approx_saved_max_tokens: 60
           } in summary.workflows
  end

  test "format_report prints a compact report" do
    report =
      %{
        total_records: 0,
        total_chars: 0,
        total_approx_tokens: 0,
        manual_chars: 0,
        estimated_saved_min_tokens: 0,
        estimated_saved_max_tokens: 0,
        commands: [],
        workflows: []
      }
      |> TokenRoi.format_report("tmp/roi.jsonl")

    assert report =~ "Token ROI telemetry"
    assert report =~ "Log: tmp/roi.jsonl"
    assert report =~ "Commands: none"
  end

  defp tmp_log_path do
    Path.join(System.tmp_dir!(), "agov_token_roi_#{System.unique_integer([:positive])}.jsonl")
  end

  defp restore_env(key, nil), do: System.delete_env(key)
  defp restore_env(key, value), do: System.put_env(key, value)
end
