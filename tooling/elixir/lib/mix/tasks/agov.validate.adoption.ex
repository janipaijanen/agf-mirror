defmodule Mix.Tasks.Agov.Validate.Adoption do
  use Mix.Task

  @shortdoc "Validate AGF adoption layout requirements"

  @impl Mix.Task
  def run(args) do
    Agov.CliSafety.ensure_not_root!()

    root = List.first(args)
    result = Agov.ValidateAdoption.validate(root)

    if Agov.ValidateReporter.report_single_result("agov.validate.adoption", result) == :error do
      exit({:shutdown, 1})
    end
  end
end
