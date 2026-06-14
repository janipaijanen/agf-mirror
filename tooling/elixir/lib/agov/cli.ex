defmodule Agov.CLI do
  @moduledoc false

  def main(args) do
    Agov.CliSafety.ensure_not_root!()

    args
    |> normalize_args()
    |> dispatch()
    |> System.halt()
  end

  defp normalize_args(["agov.coord.validate" | rest]), do: ["coord", "validate" | rest]
  defp normalize_args(["agov.plan.validate" | rest]), do: ["plan", "validate" | rest]
  defp normalize_args(args), do: args

  defp dispatch(["coord", "validate" | files]) when files != [] do
    validate_files(
      files,
      &Agov.CoordValidator.validate_file/1,
      &Agov.CoordValidator.advisory_findings/1
    )
  end

  defp dispatch(["plan", "validate" | files]) when files != [] do
    validate_files(files, &Agov.PlanValidator.validate_file/1)
  end

  defp dispatch(["--help"]) do
    print_usage()
    0
  end

  defp dispatch(["-h"]) do
    print_usage()
    0
  end

  defp dispatch(_) do
    print_usage(:stderr)
    1
  end

  defp validate_files(files, validator, advisory_fun \\ nil) do
    results = Enum.map(files, &validate_and_report(&1, validator, advisory_fun))

    if Enum.any?(results, &(&1 == :error)), do: 1, else: 0
  end

  defp validate_and_report(path, validator, advisory_fun) do
    case validator.(path) do
      {:ok, data} ->
        if advisory_fun do
          Enum.each(advisory_fun.(data), fn msg ->
            IO.puts("WARN severity: advisory #{path}: #{msg}")
          end)
        end

        IO.puts("OK  severity: blocking #{path}")
        :ok

      {:error, :file_not_found} ->
        IO.puts(:stderr, "ERR severity: blocking #{path}: file not found")
        :error

      {:error, :no_frontmatter} ->
        IO.puts(:stderr, "ERR severity: blocking #{path}: no YAML front matter found")
        :error

      {:error, errors} when is_list(errors) ->
        IO.puts(:stderr, "ERR severity: blocking #{path}:")

        Enum.each(errors, fn {msg, path_str} ->
          IO.puts(:stderr, "    #{path_str}: #{msg}")
        end)

        :error

      {:error, reason} ->
        IO.puts(:stderr, "ERR severity: blocking #{path}: #{inspect(reason)}")
        :error
    end
  end

  defp print_usage(device \\ :stdio) do
    IO.puts(device, """
    Usage:
      agov coord validate <file>...
      agov plan validate <file>...
      agov agov.coord.validate <file>...
      agov agov.plan.validate <file>...
    """)
  end
end
