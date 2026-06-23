defmodule Agov.ValidateLearnings do
  @moduledoc false

  alias Agov.ValidatorUtil

  @confidence_levels ~w(single-adopter multi-adopter longitudinal industry-corroborated)
  @corroborated_levels ~w(multi-adopter longitudinal)
  @historical_id ~r/^L\d{3}$/
  @bare_timestamp_id ~r/^\d{8}T\d{6}Z$/
  @suffixed_timestamp_id ~r/^\d{8}T\d{6}Z-[a-z0-9]{4}$/
  @pii_patterns [
    {"URL", ~r/https?:\/\/[^\s"]+/},
    {"email address", ~r/[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}/},
    {"company suffix", ~r/\b(Inc|Corp|Ltd|GmbH|LLC|Oy|Oyj|Pty|plc)\b/},
    {"internal hostname", ~r/\b[a-z0-9-]+\.(internal|local|corp|cloud)\b/i}
  ]

  def validate_file(path) do
    with {:ok, data, _content} <- ValidatorUtil.read_yaml_file(path) do
      errors =
        []
        |> Kernel.++(
          data
          |> ValidatorUtil.validate_schema("learning.schema.json")
          |> Enum.reject(&learning_id_schema_finding?/1)
        )
        |> Kernel.++(semantic_errors(data))

      if errors == [], do: {:ok, data}, else: {:error, errors}
    end
  end

  def validate_all(dir \\ nil) do
    target = dir || ValidatorUtil.learnings_dir()

    target
    |> ValidatorUtil.yaml_files()
    |> Enum.reject(&template_file?/1)
    |> Enum.map(fn file -> {file, validate_file(file)} end)
    |> split_results()
  end

  defp template_file?(file) do
    file
    |> Path.basename()
    |> String.starts_with?("_")
  end

  defp semantic_errors(data) do
    learning_id_errors(data) ++ pii_errors(data) ++ confidence_errors(data)
  end

  defp learning_id_schema_finding?(%{location: location}) when is_binary(location),
    do: String.ends_with?(location, "/id")

  defp learning_id_schema_finding?(_finding), do: false

  defp learning_id_errors(data) do
    data
    |> Map.get("learnings", [])
    |> List.wrap()
    |> Enum.with_index(1)
    |> Enum.flat_map(fn {entry, index} -> learning_id_entry_errors(entry, index) end)
    |> Kernel.++(duplicate_learning_id_errors(data))
  end

  defp learning_id_entry_errors(%{"id" => id}, index) when is_binary(id) do
    cond do
      Regex.match?(@bare_timestamp_id, id) ->
        [
          %{
            message:
              "bare timestamp learning id is transitional; new records require YYYYMMDDThhmmssZ-<4-char-random>",
            location: "learnings/#{index}/id",
            severity: :advisory
          }
        ]

      Regex.match?(@historical_id, id) or Regex.match?(@suffixed_timestamp_id, id) ->
        []

      true ->
        [
          %{
            message:
              "learning id #{inspect(id)} is invalid; use L### for historical records or YYYYMMDDThhmmssZ-<4-char-random> for new records",
            location: "learnings/#{index}/id"
          }
        ]
    end
  end

  defp learning_id_entry_errors(%{"id" => id}, index) do
    [
      %{
        message: "learning id #{inspect(id)} is invalid; expected string",
        location: "learnings/#{index}/id"
      }
    ]
  end

  defp learning_id_entry_errors(_entry, _index), do: []

  defp duplicate_learning_id_errors(data) do
    data
    |> Map.get("learnings", [])
    |> List.wrap()
    |> Enum.with_index(1)
    |> Enum.flat_map(fn
      {%{"id" => id}, index} when is_binary(id) -> [{id, index}]
      {_entry, _index} -> []
    end)
    |> Enum.group_by(&elem(&1, 0), &elem(&1, 1))
    |> Enum.flat_map(fn {id, indexes} ->
      case indexes do
        [_single] ->
          []

        duplicates ->
          [
            %{
              message:
                "duplicate learning id #{inspect(id)} at entries #{Enum.join(duplicates, ", ")}",
              location: "learnings/id"
            }
          ]
      end
    end)
  end

  defp pii_errors(data) do
    text = data |> ValidatorUtil.freetext() |> Enum.join("\n")

    @pii_patterns
    |> Enum.filter(fn {_label, regex} -> Regex.match?(regex, text) end)
    |> Enum.map(fn {label, _regex} -> %{message: "contains #{label}", location: "freetext"} end)
  end

  defp confidence_errors(data) do
    data
    |> Map.get("learnings", [])
    |> List.wrap()
    |> Enum.with_index(1)
    |> Enum.flat_map(fn
      {%{"confidence" => confidence}, index} when is_map(confidence) ->
        confidence_entry_errors(confidence, index)

      {_entry, _index} ->
        []
    end)
  end

  defp confidence_entry_errors(confidence, index) do
    level = confidence["level"]

    []
    |> maybe_add(
      level not in @confidence_levels,
      "confidence.level #{inspect(level)} is invalid",
      "learnings/#{index}/confidence/level"
    )
    |> maybe_add(
      level in @corroborated_levels and not corroborated?(confidence),
      "confidence.level #{level} requires evidence_count >= 2 or at least two evidence_refs",
      "learnings/#{index}/confidence"
    )
  end

  defp corroborated?(confidence) do
    evidence_count = confidence["evidence_count"]
    evidence_refs = confidence["evidence_refs"]

    (is_integer(evidence_count) and evidence_count >= 2) or
      (is_list(evidence_refs) and length(evidence_refs) >= 2)
  end

  defp maybe_add(errors, true, message, location),
    do: [%{message: message, location: location} | errors]

  defp maybe_add(errors, false, _message, _location), do: errors

  defp split_results(results) do
    errors =
      results
      |> Enum.flat_map(fn
        {file, {:error, file_errors}} -> [%{file: file, errors: file_errors}]
        {_file, {:ok, _data}} -> []
      end)

    if errors == [], do: {:ok, Enum.map(results, &elem(&1, 0))}, else: {:error, errors}
  end
end
