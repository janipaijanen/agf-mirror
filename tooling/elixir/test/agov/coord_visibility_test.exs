defmodule Agov.CoordVisibilityTest do
  use ExUnit.Case, async: true

  alias Agov.CoordVisibility

  defmodule StubGit do
    @behaviour Agov.CoordVisibility.GitAdapter

    @branches ["master", "feat/hidden", "feat/bridged", "feat/docs"]

    def local_branches, do: {:ok, @branches}

    def coordination_files(ref, _coord_dir) do
      case Map.fetch(files(), ref) do
        {:ok, files} -> {:ok, Map.keys(files) |> Enum.sort()}
        :error -> {:error, :unknown_ref}
      end
    end

    def read_file(ref, path) do
      case get_in(files(), [ref, path]) do
        nil -> {:error, :missing_file}
        content -> {:ok, content}
      end
    end

    defp files do
      %{
        "master" => %{
          "docs/coordination/00100-governor-reviewer.md" => decision(100, "base decision", []),
          "docs/coordination/00120-coder-tester.md" =>
            bridge(120, "bridge hidden", ["docs/coordination/00110-coder-tester.md"], ""),
          "docs/coordination/00130-coder-tester.md" =>
            bridge(130, "body bridge", [], "feat/bridged docs/coordination/00121-coder-tester.md"),
          "docs/coordination/00188-planner-architect.md" => decision(188, "master collision", [])
        },
        "feat/hidden" => %{
          "docs/coordination/00100-governor-reviewer.md" => decision(100, "base decision", []),
          "docs/coordination/00111-coder-tester.md" =>
            handoff(111, "governor-reviewer", "unbridged gate2"),
          "docs/coordination/00188-coder-tester.md" =>
            handoff(188, "governor-reviewer", "collision")
        },
        "feat/bridged" => %{
          "docs/coordination/00100-governor-reviewer.md" => decision(100, "base decision", []),
          "docs/coordination/00110-coder-tester.md" =>
            handoff(110, "governor-reviewer", "referenced bridge"),
          "docs/coordination/00121-coder-tester.md" =>
            handoff(121, "planner-architect", "body bridge")
        },
        "feat/docs" => %{
          "docs/coordination/00100-governor-reviewer.md" => decision(100, "base decision", []),
          "docs/coordination/00140-coder-tester.md" => note(140, "coder-tester", "local note")
        }
      }
    end

    defp handoff(seq, to, topic) do
      frontmatter(%{
        "kind" => "handoff",
        "from" => "coder-tester",
        "to" => to,
        "seq" => seq,
        "topic" => topic,
        "requires-ack" => true,
        "handoff" => %{
          "next_agent" => to,
          "status" => "ready",
          "task" => [topic],
          "must_read" => [],
          "do_not_touch" => []
        }
      })
    end

    defp decision(seq, topic, refs) do
      frontmatter(%{
        "kind" => "decision-record",
        "from" => "governor-reviewer",
        "to" => "all",
        "seq" => seq,
        "topic" => topic,
        "references-to" => refs
      })
    end

    defp bridge(seq, topic, refs, body) do
      decision(seq, topic, refs) <> "\n" <> body
    end

    defp note(seq, to, topic) do
      frontmatter(%{
        "kind" => "note",
        "from" => "coder-tester",
        "to" => to,
        "seq" => seq,
        "topic" => topic,
        "requires-ack" => false
      })
    end

    defp frontmatter(attrs), do: Agov.CoordVisibilityTest.frontmatter(attrs)
  end

  defmodule MissingBaseGit do
    @behaviour Agov.CoordVisibility.GitAdapter

    def local_branches, do: {:ok, ["feat/only"]}
    def coordination_files("master", _coord_dir), do: {:error, :unknown_ref}
    def coordination_files(_ref, _coord_dir), do: {:ok, []}
    def read_file(_ref, _path), do: {:error, :missing_file}
  end

  test "reports unbridged handoffs on side branches" do
    assert {:ok, report} =
             CoordVisibility.scan(git: StubGit, base_branch: "master", branches: ["feat/hidden"])

    assert [%{branch: "feat/hidden", seq: 111, to: "governor-reviewer"}] =
             Enum.filter(report.unbridged_obligations, &(&1.seq == 111))
  end

  test "does not report handoffs bridged by references-to on base branch" do
    assert {:ok, report} =
             CoordVisibility.scan(git: StubGit, base_branch: "master", branches: ["feat/bridged"])

    refute Enum.any?(report.unbridged_obligations, &(&1.seq == 110))
  end

  test "does not report handoffs bridged by body text naming branch and filename" do
    assert {:ok, report} =
             CoordVisibility.scan(git: StubGit, base_branch: "master", branches: ["feat/bridged"])

    refute Enum.any?(report.unbridged_obligations, &(&1.seq == 121))
  end

  test "ignores non-obligation branch-local notes" do
    assert {:ok, report} =
             CoordVisibility.scan(git: StubGit, base_branch: "master", branches: ["feat/docs"])

    assert report.unbridged_obligations == []
  end

  test "reports branch-vs-base sequence collisions" do
    assert {:ok, report} =
             CoordVisibility.scan(git: StubGit, base_branch: "master", branches: ["feat/hidden"])

    assert [%{seq: 188, base_file: "docs/coordination/00188-planner-architect.md"}] =
             report.sequence_collisions
  end

  test "reports duplicate seq collisions on the base branch itself" do
    files = [
      {"docs/coordination/00188-coder-tester.md", frontmatter(%{"seq" => 188})},
      {"docs/coordination/00188-planner-architect.md", frontmatter(%{"seq" => 188})}
    ]

    assert [%{seq: 188, files: files_for_seq}] = CoordVisibility.base_sequence_collisions(files)

    assert Enum.sort(files_for_seq) == [
             "docs/coordination/00188-coder-tester.md",
             "docs/coordination/00188-planner-architect.md"
           ]
  end

  test "returns a clear error when base branch is missing" do
    assert {:error, {:base_branch_unavailable, "master"}} =
             CoordVisibility.scan(git: MissingBaseGit, base_branch: "master")
  end

  test "JSON format contains stable fields" do
    assert {:ok, report} =
             CoordVisibility.scan(git: StubGit, base_branch: "master", branches: ["feat/hidden"])

    json = CoordVisibility.format_json(report)
    decoded = Jason.decode!(json)

    assert decoded["base_branch"] == "master"

    assert [%{"branch" => "feat/hidden", "seq" => 111}] =
             Enum.filter(decoded["unbridged_obligations"], &(&1["seq"] == 111))

    assert [%{"seq" => 188}] = decoded["sequence_collisions"]
  end

  test "exit code is 1 for unbridged obligations and 2 for collisions" do
    report = %{
      unbridged_obligations: [%{}],
      sequence_collisions: [],
      errors: []
    }

    assert CoordVisibility.exit_code(report) == 1

    report = %{report | sequence_collisions: [%{}]}
    assert CoordVisibility.exit_code(report) == 2
  end

  def frontmatter(attrs) do
    data =
      Map.merge(
        %{
          "kind" => "note",
          "from" => "coder-tester",
          "to" => "all",
          "seq" => 1,
          "timestamp" => "2026-06-13T00:00:00Z",
          "immutable" => true,
          "references-to" => [],
          "topic" => "fixture",
          "requires-ack" => false,
          "artifacts_owned" => []
        },
        attrs
      )

    """
    ---
    #{yaml(data)}---

    Fixture body.
    """
  end

  defp yaml(data) do
    ordered_keys = [
      "kind",
      "from",
      "to",
      "seq",
      "timestamp",
      "immutable",
      "references-to",
      "topic",
      "requires-ack",
      "artifacts_owned",
      "handoff"
    ]

    ordered_keys
    |> Enum.filter(&Map.has_key?(data, &1))
    |> Enum.map(&yaml_pair(&1, Map.fetch!(data, &1), 0))
    |> Enum.join("")
  end

  defp yaml_pair(key, value, indent) when is_map(value) do
    spaces = String.duplicate(" ", indent)
    inner = value |> Enum.map(fn {k, v} -> yaml_pair(k, v, indent + 2) end) |> Enum.join("")
    "#{spaces}#{key}:\n#{inner}"
  end

  defp yaml_pair(key, value, indent) when is_list(value) do
    spaces = String.duplicate(" ", indent)

    case value do
      [] ->
        "#{spaces}#{key}: []\n"

      items ->
        rendered = Enum.map_join(items, "", &"#{spaces}  - #{&1}\n")
        "#{spaces}#{key}:\n#{rendered}"
    end
  end

  defp yaml_pair(key, value, indent) do
    spaces = String.duplicate(" ", indent)
    "#{spaces}#{key}: #{value}\n"
  end
end
