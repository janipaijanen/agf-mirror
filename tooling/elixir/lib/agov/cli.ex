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
    validate_coord_files(files)
  end

  defp dispatch(["coord", "brief" | args]) do
    case Agov.CoordBrief.generate(args, "docs/coordination") do
      {:ok, skeleton} ->
        IO.puts(skeleton)
        0

      {:error, :readable_requires_from} ->
        IO.puts(:stderr, "ERR  --id-format readable requires --from <role>")
        1

      {:error, {:unknown_id_format, format, supported}} ->
        IO.puts(
          :stderr,
          "ERR  unknown id format: #{format}  (supported: #{Enum.join(supported, " ")})"
        )

        1

      {:error, {:invalid_args, args}} ->
        IO.puts(:stderr, "ERR  invalid option(s): #{Enum.join(args, " ")}")
        1

      {:error, {:unknown_kind, k}} ->
        IO.puts(
          :stderr,
          "ERR  unknown kind: #{k}  (supported: note handoff claim completion ack review)"
        )

        1
    end
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

    if Enum.any?(results, fn {status, _data} -> status == :error end), do: 1, else: 0
  end

  defp validate_coord_files(files) do
    results =
      Enum.map(files, fn path ->
        {path,
         validate_and_report(
           path,
           &Agov.CoordValidator.validate_file/1,
           &Agov.CoordValidator.advisory_findings/2
         )}
      end)

    valid_file_data =
      results
      |> Enum.filter(fn {_path, {_status, data}} -> is_map(data) end)
      |> Enum.map(fn {path, {_status, data}} -> {path, data} end)

    duplicate_errors = Agov.CoordValidator.duplicate_id_errors(valid_file_data)

    Enum.each(duplicate_errors, fn {path, message} ->
      IO.puts(:stderr, "ERR severity: blocking #{path}: #{message}")
    end)

    if Enum.any?(results, fn {_path, {status, _data}} -> status == :error end) or
         duplicate_errors != [] do
      1
    else
      0
    end
  end

  defp validate_and_report(path, validator, advisory_fun) do
    case validator.(path) do
      {:ok, data} ->
        if advisory_fun do
          Enum.each(call_advisory(advisory_fun, data, path), fn msg ->
            IO.puts("WARN severity: advisory #{path}: #{msg}")
          end)
        end

        IO.puts("OK  severity: blocking #{path}")
        {:ok, data}

      {:error, :file_not_found} ->
        IO.puts(:stderr, "ERR severity: blocking #{path}: file not found")
        {:error, nil}

      {:error, :no_frontmatter} ->
        IO.puts(:stderr, "ERR severity: blocking #{path}: no YAML front matter found")
        {:error, nil}

      {:error, errors} when is_list(errors) ->
        IO.puts(:stderr, "ERR severity: blocking #{path}:")

        Enum.each(errors, fn {msg, path_str} ->
          IO.puts(:stderr, "    #{path_str}: #{msg}")
        end)

        {:error, nil}

      {:error, reason} ->
        IO.puts(:stderr, "ERR severity: blocking #{path}: #{inspect(reason)}")
        {:error, nil}
    end
  end

  defp call_advisory(advisory_fun, data, path) do
    case :erlang.fun_info(advisory_fun, :arity) do
      {:arity, 2} -> advisory_fun.(data, path)
      _ -> advisory_fun.(data)
    end
  end

  defp print_usage(device \\ :stdio) do
    IO.puts(device, """
    Usage:
      agov coord validate <file>...
      agov coord brief [--id-format ulid|readable] [--from <role>] [--to <role>] [--kind <kind>] [--topic <topic>]
      agov plan validate <file>...
      agov agov.coord.validate <file>...
      agov agov.plan.validate <file>...
    """)
  end
end
