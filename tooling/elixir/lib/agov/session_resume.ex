defmodule Agov.SessionResume do
  @moduledoc false

  alias Agov.{ChannelReader, ClaimTracker, CoordConfig, HandoffTracker}

  @coord_paths ["docs/coordination"]

  def run(coord_dir, opts \\ []) do
    config = CoordConfig.load(coord_dir)
    role = Keyword.get(opts, :role) || config.role

    if missing_role?(role) do
      {:error, :role_required}
    else
      build(coord_dir, role, config, opts)
    end
  end

  defp build(coord_dir, role, config, opts) do
    git_status = Keyword.get(opts, :git_status, Agov.GitStatus)
    now = Keyword.get(opts, :now, DateTime.utc_now())
    base_branch = Keyword.get(opts, :base_branch, "master")

    case ChannelReader.scan(coord_dir) do
      {:error, :dir_not_found} ->
        {:error, :dir_not_found}

      {:ok, scan} ->
        messages = scan.messages
        plain_messages = Enum.map(messages, &elem(&1, 1))

        report = %{
          role: role,
          branch: branch(git_status),
          dirty_files: dirty_files(git_status),
          pending_handoffs: pending_handoffs(messages, role),
          claims: claims(plain_messages, role, config.from_aliases, now),
          recent_decisions: recent_decisions(messages),
          must_read: must_read(messages, role),
          warnings: warnings(git_status, base_branch),
          invalid_files: scan.invalid_files
        }

        {:ok, render(report)}
    end
  end

  defp missing_role?(nil), do: true
  defp missing_role?(""), do: true
  defp missing_role?(_), do: false

  defp branch(git_status) do
    case git_status.current_branch() do
      {:ok, value} -> value
      {:error, reason} -> "unknown (#{inspect(reason)})"
    end
  end

  defp dirty_files(git_status) do
    case git_status.dirty_files() do
      {:ok, files} -> files
      {:error, reason} -> ["unable to read git status: #{inspect(reason)}"]
    end
  end

  defp pending_handoffs(messages, role) do
    with {:ok, %{pending: pending}} <- HandoffTracker.scan(messages) do
      pending
      |> Enum.filter(&(&1.to == role))
      |> Enum.sort_by(& &1.seq, :desc)
    end
  end

  defp claims(messages, role, aliases, now) do
    resolved_role = Map.get(aliases, role, role)

    case ClaimTracker.scan(messages, now: now, from_aliases: aliases) do
      {:ok, %{active: active, stale: stale}} ->
        %{
          active: filter_claims(active, resolved_role, aliases),
          stale: filter_claims(stale, resolved_role, aliases)
        }

      {:error, errors} ->
        %{active: [], stale: [], errors: errors}
    end
  end

  defp filter_claims(entries, role, aliases) do
    entries
    |> Enum.filter(&(Map.get(aliases, &1.from, &1.from) == role))
    |> Enum.sort_by(& &1.seq, :desc)
  end

  defp recent_decisions(messages) do
    messages
    |> Enum.map(&elem(&1, 1))
    |> Enum.filter(&(&1["kind"] == "decision-record"))
    |> Enum.sort_by(& &1["seq"], :desc)
    |> Enum.take(5)
  end

  defp must_read(messages, role) do
    pending_seqs =
      messages
      |> pending_handoffs(role)
      |> MapSet.new(& &1.seq)

    messages
    |> Enum.filter(fn {_filename, data} ->
      data["kind"] == "handoff" and MapSet.member?(pending_seqs, data["seq"])
    end)
    |> Enum.flat_map(fn {_filename, data} ->
      get_in(data, ["handoff", "must_read"]) || []
    end)
    |> Enum.uniq()
  end

  defp warnings(git_status, base_branch) do
    dirty_warnings =
      case dirty_files(git_status) do
        [] -> []
        files -> ["working tree has uncommitted changes: #{Enum.join(files, ", ")}"]
      end

    branch_warnings =
      case git_status.branch_local_files(base_branch, @coord_paths) do
        {:ok, []} ->
          []

        {:ok, files} ->
          [
            "coordination files exist on this branch but not on #{base_branch}: " <>
              Enum.join(files, ", ")
          ]

        {:skip, reason} ->
          ["branch-local coordination check skipped: #{inspect(reason)}"]

        {:error, reason} ->
          ["branch-local coordination check failed: #{inspect(reason)}"]
      end

    dirty_warnings ++ branch_warnings
  end

  defp render(report) do
    [
      "Session resume",
      "Role: #{report.role}",
      "Branch: #{report.branch}",
      "",
      render_warnings(report.warnings),
      render_invalid_files(report.invalid_files),
      render_handoffs(report.role, report.pending_handoffs),
      render_claims("Open claims for #{report.role}:", report.claims.active),
      render_claims("Stale claims for #{report.role}:", report.claims.stale),
      render_decisions(report.recent_decisions),
      render_must_read(report.must_read)
    ]
    |> List.flatten()
    |> Enum.reject(&is_nil/1)
    |> Enum.join("\n")
  end

  defp render_warnings([]), do: ["Warnings: none", ""]

  defp render_warnings(warnings) do
    ["Warnings:" | Enum.map(warnings, &"  - #{&1}")] ++ [""]
  end

  defp render_invalid_files([]), do: []

  defp render_invalid_files(files) do
    ["Invalid coordination files ignored:" | Enum.map(files, &"  - #{&1}")] ++ [""]
  end

  defp render_handoffs(role, []), do: ["Pending handoffs for #{role}: none", ""]

  defp render_handoffs(role, handoffs) do
    ["Pending handoffs for #{role}:" | Enum.map(handoffs, &format_handoff/1)] ++ [""]
  end

  defp format_handoff(handoff) do
    "  - seq #{handoff.seq}  #{handoff.topic}  [#{handoff.status}]"
  end

  defp render_claims(title, []), do: [title <> " none", ""]

  defp render_claims(title, claims) do
    [title | Enum.map(claims, &format_claim/1)] ++ [""]
  end

  defp format_claim(claim) do
    expires =
      case claim.expires_on do
        nil -> ""
        value -> "  expires #{DateTime.to_iso8601(value)}"
      end

    "  - #{claim.path}  seq #{claim.seq}#{expires}"
  end

  defp render_decisions([]), do: ["Recent decisions: none", ""]

  defp render_decisions(decisions) do
    ["Recent decisions:" | Enum.map(decisions, &format_decision/1)] ++ [""]
  end

  defp format_decision(decision) do
    "  - seq #{decision["seq"]}  #{decision["topic"]}"
  end

  defp render_must_read([]), do: ["Must read: none"]

  defp render_must_read(items) do
    ["Must read:" | Enum.map(items, &"  - #{&1}")]
  end
end
