defmodule Mix.Tasks.Agov.Validate do
  use Mix.Task

  @shortdoc "Run all migrated AGF validators"

  @impl Mix.Task
  def run(args) do
    Agov.CliSafety.ensure_not_root!()

    root = List.first(args) || Agov.ValidatorUtil.root_dir()

    results = [
      {"agov.validate.learnings",
       Agov.ValidateLearnings.validate_all(Path.join(root, "learnings")), :collection},
      {"agov.validate.agents", Agov.ValidateAgents.validate_all(Path.join(root, "agents")),
       :collection},
      {"agov.validate.adoption", Agov.ValidateAdoption.validate(root), :single},
      {"agov.validate.infosec", Agov.ValidateInfosec.validate_all(Path.join(root, "learnings")),
       :collection},
      {"agov.validate.contributions",
       Agov.ValidateContributions.validate_all(Path.join(root, "learnings")), :warnings},
      {"agov.validate.framework_config",
       Agov.ValidateFrameworkConfig.validate_file(Path.join(root, "framework.yaml")), :single}
    ]

    statuses = Enum.map(results, &report/1)

    if Enum.any?(statuses, &(&1 == :error)) do
      exit({:shutdown, 1})
    end
  end

  defp report({label, result, :collection}) do
    Agov.ValidateReporter.report_collection_result(label, result)
  end

  defp report({label, result, :single}) do
    Agov.ValidateReporter.report_single_result(label, result)
  end

  defp report({label, result, :warnings}) do
    Agov.ValidateReporter.report_warning_collection(label, result)
  end
end
