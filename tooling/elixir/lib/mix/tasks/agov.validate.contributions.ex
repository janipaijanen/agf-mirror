defmodule Mix.Tasks.Agov.Validate.Contributions do
  use Mix.Task

  @shortdoc "Emit advisory contribution warnings for learning records"

  @impl Mix.Task
  def run(args) do
    Agov.CliSafety.ensure_not_root!()

    dir = List.first(args)
    result = Agov.ValidateContributions.validate_all(dir)
    Agov.ValidateReporter.report_warning_collection("agov.validate.contributions", result)
  end
end
