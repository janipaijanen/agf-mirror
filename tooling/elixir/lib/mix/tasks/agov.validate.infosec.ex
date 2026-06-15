defmodule Mix.Tasks.Agov.Validate.Infosec do
  use Mix.Task

  @shortdoc "Scan learning records for infosec-sensitive material"

  @impl Mix.Task
  def run(args) do
    Agov.CliSafety.ensure_not_root!()

    dir = List.first(args)
    result = Agov.ValidateInfosec.validate_all(dir)

    if Agov.ValidateReporter.report_collection_result("agov.validate.infosec", result) == :error do
      exit({:shutdown, 1})
    end
  end
end
