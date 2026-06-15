defmodule Agov.SessionResume do
  @moduledoc false

  alias Agov.{
    ChannelReader,
    ClaimTracker,
    CoordConfig,
    CoordIdentity,
    HandoffTracker,
    ManifestReader
  }

  @coord_paths ["docs/coordination"]

  def run(coord_dir, opts \\ []) do
    config = CoordConfig.load(coord_dir)
    role = Keyword.get(opts, :role) || config.role

    if missing_role?(role) do
      bootstrap(coord_dir, opts)
    else
      build(coord_dir, role, config, opts)
    end
  end

  defp bootstrap(coord_dir, opts) do
    git_status = Keyword.get(opts, :git_status, Agov.GitStatus)
    base_branch = Keyword.get(opts, :base_branch, "main")
    manifest_path = Keyword.get(opts, :manifest_path)

    case ChannelReader.scan(coord_dir) do
      {:error, :dir_not_found} ->
        {:error, :dir_not_found}

      {:ok, scan} ->
        messages = scan.messages

        report = %{
          branch: branch(git_status),
          warnings: warnings(git_status, base_branch),
          active_roles: active_roles(manifest_path),
          pending_handoffs: pending_handoffs(messages),
          recent_decisions: recent_decisions(messages),
          invalid_files: scan.invalid_files,
          base_branch: base_branch
        }

        {:ok, render_bootstrap(report)}
    end
  end

  defp build(coord_dir, role, config, opts) do
    git_status = Keyword.get(opts, :git_status, Agov.GitStatus)
    now = Keyword.get(opts, :now, DateTime.utc_now())
    base_branch = Keyword.get(opts, :base_branch, "main")

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

  defp active_roles(nil), do: :unavailable

  defp active_roles(manifest_path) do
    case ManifestReader.load(manifest_path) do
      {:ok, agents} ->
        agents
        |> Enum.filter(fn {_role, data} -> data.status == "active" end)
        |> Enum.map(&elem(&1, 0))
        |> Enum.sort()

      {:error, _} ->
        :unavailable
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
      |> Enum.sort_by(& &1.sort_key, :desc)
    end
  end

  defp pending_handoffs(messages) do
    with {:ok, %{pending: pending}} <- HandoffTracker.scan(messages) do
      Enum.sort_by(pending, & &1.sort_key, :desc)
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
    |> Enum.sort_by(&claim_sort_key/1, :desc)
  end

  defp recent_decisions(messages) do
    messages
    |> Enum.filter(fn {_file, data} -> data["kind"] == "decision-record" end)
    |> Enum.sort_by(fn {file, data} -> CoordIdentity.sort_key(file, data) end, :desc)
    |> Enum.map(fn {_file, data} -> data end)
    |> Enum.take(5)
  end

  defp claim_sort_key(claim) do
    {claim.claimed_at || "", to_string(claim.id || claim.seq || "")}
  end

  defp must_read(messages, role) do
    pending_files =
      messages
      |> pending_handoffs(role)
      |> MapSet.new(& &1.file)

    messages
    |> Enum.filter(fn {_filename, data} ->
      data["kind"] == "handoff"
    end)
    |> Enum.filter(fn {filename, _data} ->
      MapSet.member?(pending_files, filename)
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

  defp render_bootstrap(report) do
    [
      "Session bootstrap",
      "Role: not selected",
      "Branch: #{report.branch}",
      "",
      render_warnings(report.warnings),
      render_invalid_files(report.invalid_files),
      render_active_roles(report.active_roles, report.base_branch),
      render_pending_by_role(report.pending_handoffs),
      render_decisions(report.recent_decisions),
      "Next step: rerun with the role you are playing, for example:",
      "  mix agov.session.resume --role coder-tester --base-branch #{report.base_branch}"
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

  defp render_active_roles(:unavailable, _base_branch) do
    [
      "Active roles: unavailable (manifest not found)",
      "Choose a role from AGENTS.md or agf/agents/MANIFEST.yaml.",
      ""
    ]
  end

  defp render_active_roles([], _base_branch), do: ["Active roles: none", ""]

  defp render_active_roles(roles, base_branch) do
    role_lines =
      Enum.map(roles, fn role ->
        "  - #{role}  -> mix agov.session.resume --role #{role} --base-branch #{base_branch}"
      end)

    ["Active roles:" | role_lines] ++ [""]
  end

  defp render_pending_by_role([]), do: ["Pending handoffs by role: none", ""]

  defp render_pending_by_role(handoffs) do
    lines =
      handoffs
      |> Enum.group_by(& &1.to)
      |> Enum.sort_by(fn {role, _items} -> role end)
      |> Enum.flat_map(fn {role, items} ->
        [
          "  #{role}:"
          | Enum.map(items, &"    - #{&1.display_id}  #{&1.topic}  [#{&1.status}]")
        ]
      end)

    ["Pending handoffs by role:" | lines] ++ [""]
  end

  defp format_handoff(handoff) do
    "  - #{handoff.display_id}  #{handoff.topic}  [#{handoff.status}]"
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

    label =
      cond do
        is_binary(Map.get(claim, :id)) -> "id #{String.slice(claim.id, 0, 8)}"
        not is_nil(claim.seq) -> "seq #{claim.seq}"
        true -> "id <unknown>"
      end

    "  - #{claim.path}  #{label}#{expires}"
  end

  defp render_decisions([]), do: ["Recent decisions: none", ""]

  defp render_decisions(decisions) do
    ["Recent decisions:" | Enum.map(decisions, &format_decision/1)] ++ [""]
  end

  defp format_decision(decision) do
    "  - #{CoordIdentity.display(decision)}  #{decision["topic"]}"
  end

  defp render_must_read([]), do: ["Must read: none"]

  defp render_must_read(items) do
    ["Must read:" | Enum.map(items, &"  - #{&1}")]
  end
end
