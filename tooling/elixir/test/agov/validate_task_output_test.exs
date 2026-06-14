defmodule Agov.ValidateTaskOutputTest do
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

  test "coord validation reports blocking severity and advisory warnings" do
    path = fixture("valid_note_no_model.md")

    Mix.Tasks.Agov.Coord.Validate.run([path])

    assert_received {:mix_shell, :info, ["WARN severity: advisory " <> warning]}
    assert warning =~ path
    assert warning =~ "model"

    assert_received {:mix_shell, :info, ["OK  severity: blocking " <> ^path]}
  end

  test "plan validation reports blocking severity" do
    path = fixture("valid_plan.md")

    Mix.Tasks.Agov.Plan.Validate.run([path])

    assert_received {:mix_shell, :info, ["OK  severity: blocking " <> ^path]}
  end

  defp fixture(name), do: Path.join(@fixtures_dir, name)
end
