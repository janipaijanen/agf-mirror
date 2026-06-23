defmodule Agov.ValidateMixTasksTest do
  use ExUnit.Case, async: false

  @fixtures_dir Path.join([__DIR__, "..", "fixtures"])

  setup do
    previous_shell = Mix.shell()
    Mix.shell(Mix.Shell.Process)

    on_exit(fn ->
      Mix.shell(previous_shell)
    end)

    :ok
  end

  test "learning task prints OK records" do
    dir = tmp_dir("learning-task")

    File.cp!(
      Path.join(@fixtures_dir, "valid_learning.yaml"),
      Path.join(dir, "valid_learning.yaml")
    )

    Mix.Tasks.Agov.Validate.Learnings.run([dir])

    assert_received {:mix_shell, :info, ["OK " <> path]}
    assert path =~ "valid_learning.yaml"
    assert_received {:mix_shell, :info, ["OK agov.validate.learnings"]}
  end

  test "learning task prints WARN records and exits cleanly for bare timestamp ids" do
    dir = tmp_dir("learning-task-warning")

    File.write!(Path.join(dir, "learning.yaml"), """
    project:
      codename: fixture
    learnings:
      - id: 20260621T150000Z
        title: Bare timestamp
        description: Historical transition record.
    """)

    Mix.Tasks.Agov.Validate.Learnings.run([dir])

    assert_received {:mix_shell, :info, ["WARN " <> warning]}
    assert warning =~ "bare timestamp learning id"
    assert_received {:mix_shell, :info, ["OK agov.validate.learnings"]}
  end

  test "contribution task prints WARN records and exits cleanly" do
    dir = tmp_dir("contribution-task")

    File.write!(Path.join(dir, "learning.yaml"), """
    project:
      codename: fixture
    learnings:
      - title: Quiet rollout
        description: No issues were observed.
    """)

    Mix.Tasks.Agov.Validate.Contributions.run([dir])

    assert_received {:mix_shell, :info, ["WARN " <> warning]}
    assert warning =~ "absence-of-incident"
    assert_received {:mix_shell, :info, ["OK agov.validate.contributions"]}
  end

  defp tmp_dir(name) do
    path = Path.join(System.tmp_dir!(), "agov-#{name}-#{System.unique_integer([:positive])}")
    File.mkdir_p!(path)
    ExUnit.Callbacks.on_exit(fn -> File.rm_rf!(path) end)
    path
  end
end
