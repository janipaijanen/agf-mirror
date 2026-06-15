defmodule Agov.ValidateContributions do
  @moduledoc false

  alias Agov.ValidatorUtil

  @absence ~r/no\s+(issues?|problems?|failures?)|without\s+(any\s+)?(issues?|problems?)/i
  @relaxation ~r/(remove|skip|bypass|weaken|relax).{0,40}(rule|gate|review|check|boundary|validation)/i

  def validate_file(path) do
    with {:ok, data, _content} <- ValidatorUtil.read_yaml_file(path) do
      warnings = warning_findings(data)
      {:ok, warnings}
    end
  end

  def validate_all(dir \\ nil) do
    target = dir || ValidatorUtil.learnings_dir()

    warnings =
      target
      |> ValidatorUtil.yaml_files()
      |> Enum.map(fn file -> {file, validate_file(file)} end)
      |> Enum.flat_map(fn
        {file, {:ok, file_warnings}} -> Enum.map(file_warnings, &Map.put(&1, :file, file))
        {_file, {:error, _errors}} -> []
      end)

    {:ok, warnings}
  end

  defp warning_findings(data) do
    text = data |> ValidatorUtil.freetext() |> Enum.join("\n")

    []
    |> add_match(
      Regex.match?(@absence, text),
      "absence-of-incident claim requires evidence",
      "freetext"
    )
    |> add_match(
      Regex.match?(@relaxation, text),
      "governance relaxation proposal requires evidence",
      "freetext"
    )
    |> Kernel.++(evidence_quality_warnings(data))
    |> Enum.reverse()
  end

  defp evidence_quality_warnings(data) do
    data
    |> Map.get("learnings", [])
    |> List.wrap()
    |> Enum.with_index(1)
    |> Enum.flat_map(fn
      {%{"confidence" => %{"level" => level} = confidence}, index}
      when level in ["multi-adopter", "longitudinal"] ->
        count = confidence["evidence_count"]
        refs = confidence["evidence_refs"]

        if count == 1 or refs in [nil, []] do
          [
            %{
              message: "weak evidence for #{level} confidence",
              location: "learnings/#{index}/confidence"
            }
          ]
        else
          []
        end

      _entry ->
        []
    end)
  end

  defp add_match(findings, true, message, location),
    do: [%{message: message, location: location} | findings]

  defp add_match(findings, false, _message, _location), do: findings
end
