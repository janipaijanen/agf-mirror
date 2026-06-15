defmodule Mix.Tasks.Agov.Validate.Agents do
  use Mix.Task

  @shortdoc "Validate AGF agent definitions"

  @impl Mix.Task
  def run(args) do
    Agov.CliSafety.ensure_not_root!()

    dir = List.first(args)
    result = Agov.ValidateAgents.validate_all(dir)

    if Agov.ValidateReporter.report_collection_result("agov.validate.agents", result) == :error do
      exit({:shutdown, 1})
    end
  end
end
