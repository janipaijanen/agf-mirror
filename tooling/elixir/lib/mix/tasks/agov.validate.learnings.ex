defmodule Mix.Tasks.Agov.Validate.Learnings do
  use Mix.Task

  @shortdoc "Validate AGF learning records"

  @impl Mix.Task
  def run(args) do
    Agov.CliSafety.ensure_not_root!()

    dir = List.first(args)
    result = Agov.ValidateLearnings.validate_all(dir)

    if Agov.ValidateReporter.report_collection_result("agov.validate.learnings", result) == :error do
      exit({:shutdown, 1})
    end
  end
end
