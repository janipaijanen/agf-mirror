defmodule Agov.SessionResumeTest do
  use ExUnit.Case, async: true

  alias Agov.SessionResume

  defmodule CleanGit do
    @behaviour Agov.GitStatus

    def current_branch, do: {:ok, "feat/session-resume"}
    def dirty_files, do: {:ok, []}
    def branch_local_files(_base_branch, _paths), do: {:ok, []}
  end

  defmodule NoisyGit do
    @behaviour Agov.GitStatus

    def current_branch, do: {:ok, "feat/session-resume"}
    def dirty_files, do: {:ok, ["docs/coordination/00188-coder-tester.md"]}

    def branch_local_files("master", ["docs/coordination"]) do
      {:ok, ["docs/coordination/00188-coder-tester.md"]}
    end
  end

  defmodule SkippingGit do
    @behaviour Agov.GitStatus

    def current_branch, do: {:ok, "feat/session-resume"}
    def dirty_files, do: {:ok, []}
    def branch_local_files(_base_branch, _paths), do: {:skip, :base_branch_unavailable}
  end

  test "infers role from agov-config.yaml and shows pending handoffs for that role" do
    coord_dir =
      new_coord_dir!(%{
        "agov-config.yaml" => "current_role: coder-tester\n",
        "00100-planner-architect.md" => handoff(100, "coder-tester", "implement session resume"),
        "00101-planner-architect.md" => handoff(101, "documentation-writer", "write docs")
      })

    assert {:ok, output} =
             SessionResume.run(coord_dir,
               git_status: CleanGit,
               now: ~U[2026-06-13 10:00:00Z],
               base_branch: "master"
             )

    assert output =~ "Role: coder-tester"
    assert output =~ "Branch: feat/session-resume"
    assert output =~ "Pending handoffs for coder-tester:"
    assert output =~ "seq 100  implement session resume"
    refute output =~ "write docs"
  end

  test "role option overrides config role" do
    coord_dir =
      new_coord_dir!(%{
        "agov-config.yaml" => "current_role: planner-architect\n",
        "00100-planner-architect.md" => handoff(100, "coder-tester", "coder task"),
        "00101-coder-tester.md" => handoff(101, "planner-architect", "planner task")
      })

    assert {:ok, output} =
             SessionResume.run(coord_dir,
               role: "coder-tester",
               git_status: CleanGit,
               now: ~U[2026-06-13 10:00:00Z],
               base_branch: "master"
             )

    assert output =~ "Role: coder-tester"
    assert output =~ "seq 100  coder task"
    refute output =~ "planner task"
  end

  test "shows active and stale claims owned by the selected role" do
    coord_dir =
      new_coord_dir!(%{
        "agov-config.yaml" => "current_role: coder-tester\n",
        "00100-coder-tester.md" =>
          claim(100, "coder-tester", "tooling/elixir/lib/agov/session_resume.ex", nil),
        "00101-coder-tester.md" =>
          claim(
            101,
            "coder-tester",
            "docs/coordination/00101-coder-tester.md",
            "2026-01-01T00:00:00Z"
          ),
        "00102-planner-architect.md" =>
          claim(102, "planner-architect", "docs/plans/other.md", nil)
      })

    assert {:ok, output} =
             SessionResume.run(coord_dir,
               git_status: CleanGit,
               now: ~U[2026-06-13 10:00:00Z],
               base_branch: "master"
             )

    assert output =~ "Open claims for coder-tester:"
    assert output =~ "tooling/elixir/lib/agov/session_resume.ex  seq 100"
    assert output =~ "Stale claims for coder-tester:"
    assert output =~ "docs/coordination/00101-coder-tester.md  seq 101"
    refute output =~ "docs/plans/other.md"
  end

  test "shows recent decisions and omits resolved handoffs from pending list" do
    coord_dir =
      new_coord_dir!(%{
        "agov-config.yaml" => "current_role: coder-tester\n",
        "00100-planner-architect.md" => handoff(100, "coder-tester", "old handoff"),
        "00101-governor-reviewer.md" =>
          decision(101, "approve old handoff", ["00100-planner-architect.md"]),
        "00102-planner-architect.md" => handoff(102, "coder-tester", "new handoff")
      })

    assert {:ok, output} =
             SessionResume.run(coord_dir,
               git_status: CleanGit,
               now: ~U[2026-06-13 10:00:00Z],
               base_branch: "master"
             )

    assert output =~ "seq 102  new handoff"
    refute output =~ "seq 100  old handoff"
    assert output =~ "Recent decisions:"
    assert output =~ "seq 101  approve old handoff"
  end

  test "surfaces dirty tree and hidden branch-local coordination warnings" do
    coord_dir =
      new_coord_dir!(%{
        "agov-config.yaml" => "current_role: coder-tester\n",
        "00100-planner-architect.md" => handoff(100, "coder-tester", "visible handoff")
      })

    assert {:ok, output} =
             SessionResume.run(coord_dir,
               git_status: NoisyGit,
               now: ~U[2026-06-13 10:00:00Z],
               base_branch: "master"
             )

    assert output =~ "Warnings:"
    assert output =~ "working tree has uncommitted changes"
    assert output =~ "docs/coordination/00188-coder-tester.md"
    assert output =~ "coordination files exist on this branch but not on master"
  end

  test "reports skipped hidden-file check without failing the resume" do
    coord_dir =
      new_coord_dir!(%{
        "agov-config.yaml" => "current_role: coder-tester\n"
      })

    assert {:ok, output} =
             SessionResume.run(coord_dir,
               git_status: SkippingGit,
               now: ~U[2026-06-13 10:00:00Z],
               base_branch: "missing-base"
             )

    assert output =~ "Warnings:"
    assert output =~ "branch-local coordination check skipped"
  end

  test "without a role, prints a cold-start bootstrap view" do
    coord_dir =
      new_coord_dir!(%{
        "agov-config.yaml" => "from_aliases: {}\n",
        "00100-planner-architect.md" => handoff(100, "coder-tester", "coder task"),
        "00101-coder-tester.md" => handoff(101, "governor-reviewer", "review task")
      })

    manifest_path =
      new_file!("""
      agents:
        coder-tester:
          status: active
        governor-reviewer:
          status: active
        build-engineer:
          status: suspended
      """)

    assert {:ok, output} =
             SessionResume.run(coord_dir,
               git_status: CleanGit,
               now: ~U[2026-06-13 10:00:00Z],
               base_branch: "master",
               manifest_path: manifest_path
             )

    assert output =~ "Session bootstrap"
    assert output =~ "Role: not selected"
    assert output =~ "Active roles:"
    assert output =~ "coder-tester  -> mix agov.session.resume --role coder-tester"
    assert output =~ "governor-reviewer  -> mix agov.session.resume --role governor-reviewer"
    refute output =~ "build-engineer  ->"
    assert output =~ "Pending handoffs by role:"
    assert output =~ "coder-tester:"
    assert output =~ "governor-reviewer:"
    assert output =~ "Next step: rerun with the role you are playing"
  end

  defp new_coord_dir!(files) do
    dir =
      Path.join(System.tmp_dir!(), "agov_session_resume_#{System.unique_integer([:positive])}")

    File.rm_rf!(dir)
    File.mkdir_p!(dir)

    Enum.each(files, fn {name, content} ->
      File.write!(Path.join(dir, name), content)
    end)

    dir
  end

  defp new_file!(content) do
    path =
      Path.join(
        System.tmp_dir!(),
        "agov_session_resume_#{System.unique_integer([:positive])}.yaml"
      )

    File.write!(path, content)
    path
  end

  defp handoff(seq, to, topic) do
    """
    ---
    kind: handoff
    from: planner-architect
    to: #{to}
    seq: #{seq}
    timestamp: 2026-06-13T00:00:00Z
    immutable: true
    references-to: []
    topic: #{topic}
    requires-ack: true
    artifacts_owned: []
    handoff:
      next_agent: #{to}
      status: ready
      task:
        - #{topic}
      must_read:
        - docs/plans/example.md
      do_not_touch: []
    ---

    Handoff fixture.
    """
  end

  defp claim(seq, from, path, expires_on) do
    expires =
      case expires_on do
        nil -> ""
        value -> "expires_on: \"#{value}\"\n"
      end

    """
    ---
    kind: claim
    from: #{from}
    to: governor-reviewer
    seq: #{seq}
    timestamp: 2026-06-13T00:00:00Z
    immutable: true
    references-to: []
    topic: claim #{path}
    requires-ack: false
    artifacts_owned:
      - #{path}
    #{expires}---

    Claim fixture.
    """
  end

  defp decision(seq, topic, refs) do
    refs_yaml = Enum.map_join(refs, "\n", &"  - #{&1}")

    """
    ---
    kind: decision-record
    from: governor-reviewer
    to: all
    seq: #{seq}
    timestamp: 2026-06-13T00:00:00Z
    immutable: true
    references-to:
    #{refs_yaml}
    topic: #{topic}
    requires-ack: false
    artifacts_owned: []
    ---

    Decision fixture.
    """
  end
end
