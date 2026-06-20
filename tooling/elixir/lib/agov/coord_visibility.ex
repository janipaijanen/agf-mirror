defmodule Agov.CoordVisibility do
  @moduledoc false

  alias Agov.CoordIdentity

  @coord_dir "docs/coordination"

  defmodule GitAdapter do
    @moduledoc false

    @callback local_branches() :: {:ok, [String.t()]} | {:error, term()}
    @callback coordination_files(String.t(), String.t()) :: {:ok, [String.t()]} | {:error, term()}
    @callback read_file(String.t(), String.t()) :: {:ok, String.t()} | {:error, term()}
  end

  defmodule Git do
    @moduledoc false
    @behaviour Agov.CoordVisibility.GitAdapter

    def local_branches do
      case git(["for-each-ref", "--format=%(refname:short)", "refs/heads"]) do
        {:ok, output} ->
          branches =
            output
            |> String.split("\n", trim: true)
            |> Enum.sort()

          {:ok, branches}

        {:error, _} = error ->
          error
      end
    end

    def coordination_files(ref, coord_dir) do
      with {:ok, root} <- git_root(),
           {:ok, path} <- repo_relative_path(root, coord_dir),
           {:ok, output} <- git_in(root, ["ls-tree", "-r", "--name-only", ref, "--", path]) do
        files =
          output
          |> String.split("\n", trim: true)
          |> Enum.filter(&String.ends_with?(&1, ".md"))
          |> Enum.sort()

        {:ok, files}
      end
    end

    def read_file(ref, path) do
      with {:ok, root} <- git_root() do
        git_in(root, ["show", "#{ref}:#{path}"])
      end
    end

    defp repo_relative_path(root, path) do
      expanded = Path.expand(path)

      if String.starts_with?(expanded, root) do
        {:ok, Path.relative_to(expanded, root)}
      else
        {:ok, path}
      end
    end

    defp git_root, do: git(["rev-parse", "--show-toplevel"])

    defp git(args) do
      case System.cmd("git", args, stderr_to_stdout: true) do
        {output, 0} -> {:ok, String.trim_trailing(output)}
        {output, status} -> {:error, {:git_failed, status, String.trim(output)}}
      end
    end

    defp git_in(root, args) do
      case System.cmd("git", args, cd: root, stderr_to_stdout: true) do
        {output, 0} -> {:ok, String.trim_trailing(output)}
        {output, status} -> {:error, {:git_failed, status, String.trim(output)}}
      end
    end
  end

  def scan(opts \\ []) do
    git = Keyword.get(opts, :git, Git)
    base_branch = Keyword.get(opts, :base_branch, "master")
    coord_dir = Keyword.get(opts, :dir, @coord_dir)

    with {:ok, base_files} <- base_files(git, base_branch, coord_dir),
         {:ok, base_messages} <- load_messages(git, base_branch, base_files),
         {:ok, branches} <- branches_to_scan(git, base_branch, Keyword.get(opts, :branches)),
         {:ok, branch_reports} <-
           scan_branches(git, branches, base_branch, coord_dir, base_messages) do
      report = %{
        base_branch: base_branch,
        scanned_branches: branches,
        unbridged_obligations: branch_reports |> Enum.flat_map(& &1.unbridged_obligations),
        sequence_collisions:
          base_sequence_collisions(base_messages) ++
            (branch_reports |> Enum.flat_map(& &1.sequence_collisions)),
        errors: []
      }

      {:ok, report}
    end
  end

  def base_sequence_collisions(file_messages) do
    file_messages
    |> Enum.flat_map(&normalize_file_message/1)
    |> Enum.reject(fn {file, data} ->
      CoordIdentity.ulid_filename?(file) or CoordIdentity.readable_filename?(file) or
        is_nil(data["seq"])
    end)
    |> Enum.group_by(fn {_file, data} -> data["seq"] end, fn {file, _data} -> file end)
    |> Enum.filter(fn {_seq, files} -> length(files) > 1 end)
    |> Enum.map(fn {seq, files} ->
      %{seq: seq, files: Enum.sort(files), type: :base_duplicate_seq}
    end)
    |> Enum.sort_by(& &1.seq)
  end

  def format_json(report) do
    report
    |> normalize_for_json()
    |> Jason.encode!()
  end

  def format_text(report) do
    cond do
      report.sequence_collisions != [] or report.unbridged_obligations != [] ->
        [
          collision_lines(report.sequence_collisions),
          obligation_lines(report.unbridged_obligations)
        ]
        |> List.flatten()
        |> Enum.reject(&(&1 == ""))
        |> Enum.join("\n")

      true ->
        "OK coordination visibility clean relative to #{report.base_branch}"
    end
  end

  def exit_code(report) do
    cond do
      Map.get(report, :sequence_collisions, []) != [] or Map.get(report, :errors, []) != [] -> 2
      Map.get(report, :unbridged_obligations, []) != [] -> 1
      true -> 0
    end
  end

  defp base_files(git, base_branch, coord_dir) do
    case git.coordination_files(base_branch, coord_dir) do
      {:ok, files} -> {:ok, files}
      {:error, _} -> {:error, {:base_branch_unavailable, base_branch}}
    end
  end

  defp branches_to_scan(git, base_branch, nil) do
    with {:ok, branches} <- git.local_branches() do
      {:ok, branches |> Enum.reject(&(&1 == base_branch)) |> Enum.sort()}
    end
  end

  defp branches_to_scan(_git, _base_branch, branches), do: {:ok, branches}

  defp scan_branches(git, branches, base_branch, coord_dir, base_messages) do
    Enum.reduce_while(branches, {:ok, []}, fn branch, {:ok, acc} ->
      case scan_branch(git, branch, base_branch, coord_dir, base_messages) do
        {:ok, report} -> {:cont, {:ok, [report | acc]}}
        {:error, _} = error -> {:halt, error}
      end
    end)
    |> case do
      {:ok, reports} -> {:ok, Enum.reverse(reports)}
      error -> error
    end
  end

  defp scan_branch(git, branch, _base_branch, coord_dir, base_messages) do
    base_file_set = base_messages |> Enum.map(&elem(&1, 0)) |> MapSet.new()

    with {:ok, branch_files} <- git.coordination_files(branch, coord_dir),
         branch_local_files = Enum.reject(branch_files, &MapSet.member?(base_file_set, &1)),
         {:ok, branch_local_messages} <- load_messages(git, branch, branch_local_files) do
      obligations =
        branch_local_messages
        |> Enum.filter(fn {_file, data} -> obligation?(data) end)
        |> Enum.reject(fn {file, data} -> bridged?(base_messages, branch, file, data) end)
        |> Enum.map(fn {file, data} -> obligation_entry(branch, file, data) end)

      {:ok,
       %{
         branch: branch,
         unbridged_obligations: obligations,
         sequence_collisions: sequence_collisions(branch, branch_local_messages, base_messages)
       }}
    end
  end

  defp normalize_file_message({file, data}) when is_map(data), do: [{file, data}]

  defp normalize_file_message({file, content}) when is_binary(content) do
    case parse_frontmatter(content) do
      {:ok, data} -> [{file, data}]
      {:error, _} -> []
    end
  end

  defp load_messages(git, ref, files) do
    Enum.reduce_while(files, {:ok, []}, fn file, {:ok, acc} ->
      with {:ok, content} <- git.read_file(ref, file),
           {:ok, data} <- parse_frontmatter(content) do
        {:cont, {:ok, [{file, data} | acc]}}
      else
        {:error, _reason} ->
          {:cont, {:ok, acc}}
      end
    end)
    |> case do
      {:ok, messages} -> {:ok, Enum.reverse(messages)}
      error -> error
    end
  end

  defp parse_frontmatter(content) do
    with {:ok, yaml} <- Agov.CoordValidator.extract_frontmatter(content),
         {:ok, data} <- YamlElixir.read_from_string(yaml) do
      {:ok, Map.put(data, "_body", body(content))}
    end
  end

  defp body(content) do
    case String.split(content, "---", parts: 3) do
      [_before, _yaml, rest] -> rest
      _ -> ""
    end
  end

  defp obligation?(%{"kind" => "handoff"}), do: true
  defp obligation?(%{"requires-ack" => true}), do: true
  defp obligation?(%{"to" => "all"}), do: true

  defp obligation?(%{"kind" => kind, "from" => from, "to" => to})
       when kind in ["review", "decision-record", "note"] do
    from != to
  end

  defp obligation?(_data), do: false

  defp bridged?(base_messages, branch, file, data) do
    Enum.any?(base_messages, fn {base_file, base_data} ->
      CoordIdentity.later?(base_file, base_data, file, data) and
        (file in (base_data["references-to"] || []) or
           String.contains?(base_data["_body"] || "", file) or
           (String.contains?(base_data["_body"] || "", branch) and
              String.contains?(base_data["_body"] || "", Path.basename(file))))
    end)
  end

  defp obligation_entry(branch, file, data) do
    %{
      branch: branch,
      file: file,
      seq: data["seq"],
      kind: data["kind"],
      to: data["to"],
      topic: data["topic"],
      bridge: :missing
    }
  end

  defp sequence_collisions(branch, branch_messages, base_messages) do
    base_by_seq =
      base_messages
      |> Enum.reject(fn {file, data} ->
        CoordIdentity.ulid_filename?(file) or CoordIdentity.readable_filename?(file) or
          is_nil(data["seq"])
      end)
      |> Enum.group_by(fn {_file, data} -> data["seq"] end)

    branch_messages
    |> Enum.reject(fn {file, data} ->
      CoordIdentity.ulid_filename?(file) or CoordIdentity.readable_filename?(file) or
        is_nil(data["seq"])
    end)
    |> Enum.flat_map(fn {file, data} ->
      base_by_seq
      |> Map.get(data["seq"], [])
      |> Enum.reject(fn {base_file, _base_data} -> base_file == file end)
      |> Enum.map(fn {base_file, _base_data} ->
        %{
          type: :branch_base_seq_collision,
          seq: data["seq"],
          branch: branch,
          branch_file: file,
          base_file: base_file
        }
      end)
    end)
    |> Enum.sort_by(&{&1.seq, &1.branch_file})
  end

  defp collision_lines([]), do: []

  defp collision_lines(collisions) do
    ["ERR sequence collision" | Enum.flat_map(collisions, &format_collision/1)]
  end

  defp format_collision(%{type: :base_duplicate_seq, seq: seq, files: files}) do
    ["  seq: #{seq}" | Enum.map(files, &"  base-file: #{&1}")]
  end

  defp format_collision(collision) do
    [
      "  seq: #{collision.seq}",
      "  base: #{collision.base_file}",
      "  branch: #{collision.branch}",
      "  branch-file: #{collision.branch_file}"
    ]
  end

  defp obligation_lines([]), do: []

  defp obligation_lines(obligations) do
    [
      "WARN branch-local coordination obligations not visible on base branch"
      | Enum.flat_map(obligations, &format_obligation/1)
    ]
  end

  defp format_obligation(obligation) do
    [
      "  branch: #{obligation.branch}",
      "  file: #{obligation.file}",
      "  seq: #{obligation.seq}",
      "  kind: #{obligation.kind}",
      "  to: #{obligation.to}",
      "  topic: #{obligation.topic}",
      "  bridge: missing"
    ]
  end

  defp normalize_for_json(report) do
    %{
      base_branch: report.base_branch,
      scanned_branches: report.scanned_branches,
      unbridged_obligations: Enum.map(report.unbridged_obligations, &stringify_bridge/1),
      sequence_collisions: report.sequence_collisions,
      errors: report.errors,
      exit_code: exit_code(report)
    }
  end

  defp stringify_bridge(entry), do: Map.update!(entry, :bridge, &to_string/1)
end
