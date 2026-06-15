defmodule Mix.Tasks.Agov.Validate.FrameworkConfig do
  use Mix.Task

  @shortdoc "Validate optional AGF framework.yaml"

  @impl Mix.Task
  def run(args) do
    Agov.CliSafety.ensure_not_root!()

    result =
      case args do
        [path | _rest] -> Agov.ValidateFrameworkConfig.validate_file(path)
        [] -> Agov.ValidateFrameworkConfig.validate_default()
      end

    if Agov.ValidateReporter.report_single_result("agov.validate.framework_config", result) ==
         :error do
      exit({:shutdown, 1})
    end
  end
end
