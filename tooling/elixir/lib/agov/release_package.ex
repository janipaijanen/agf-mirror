defmodule Agov.ReleasePackage do
  @moduledoc false

  @default_root "../.."
  @target_platform "linux-x86_64"
  @release_name "agov"

  def default_root, do: @default_root

  def build_tooling(opts \\ []) do
    root = opts |> Keyword.get(:root, @default_root) |> Path.expand()
    output_dir = opts |> Keyword.get(:output_dir, Path.join(root, "dist")) |> Path.expand()
    version_file = Keyword.get(opts, :version_file, Path.join(root, "agf/VERSION"))
    app_dir = opts |> Keyword.get(:app_dir, Path.join(root, "tooling/elixir")) |> Path.expand()
    release_dir = Keyword.get(opts, :release_dir)
    run_release? = Keyword.get(opts, :run_release?, true)

    with {:ok, version} <- read_version(version_file),
         :ok <- File.mkdir_p(output_dir),
         :ok <- maybe_run_release(app_dir, run_release?),
         {:ok, release_dir} <- resolve_release_dir(app_dir, release_dir),
         {:ok, archive_path} <- write_tooling_archive(release_dir, output_dir, version),
         {:ok, checksum, checksum_path} <- write_checksum(archive_path),
         {:ok, provenance_path} <-
           write_tooling_provenance(output_dir, archive_path, checksum, version) do
      {:ok,
       %{
         version: version,
         archive_path: archive_path,
         checksum_path: checksum_path,
         checksum: checksum,
         provenance_path: provenance_path
       }}
    end
  end

  def build_bundle(opts \\ []) do
    root = opts |> Keyword.get(:root, @default_root) |> Path.expand()
    output_dir = opts |> Keyword.get(:output_dir, Path.join(root, "dist")) |> Path.expand()
    version_file = Keyword.get(opts, :version_file, Path.join(root, "agf/VERSION"))

    with {:ok, version} <- read_version(version_file),
         :ok <- File.mkdir_p(output_dir),
         {:ok, inputs} <- bundle_inputs(output_dir, version),
         {:ok, archive_path} <- write_bundle_archive(output_dir, version, inputs),
         {:ok, checksum, checksum_path} <- write_checksum(archive_path) do
      {:ok,
       %{
         version: version,
         archive_path: archive_path,
         checksum_path: checksum_path,
         checksum: checksum,
         inputs: inputs
       }}
    end
  end

  def format_summary({:ok, %{provenance_path: provenance_path} = result}) do
    """
    OK  tooling release archive built
    Archive:    #{Path.relative_to_cwd(result.archive_path)}
    Checksum:   #{Path.relative_to_cwd(result.checksum_path)}
    Provenance: #{Path.relative_to_cwd(provenance_path)}
    """
    |> String.trim_trailing()
  end

  def format_summary({:ok, result}) do
    """
    OK  bundle archive built
    Archive:  #{Path.relative_to_cwd(result.archive_path)}
    Checksum: #{Path.relative_to_cwd(result.checksum_path)}
    Inputs:   #{length(result.inputs)}
    """
    |> String.trim_trailing()
  end

  def format_summary({:error, errors}) when is_list(errors) do
    errors |> Enum.map(&"ERR #{&1}") |> Enum.join("\n")
  end

  def format_summary({:error, error}), do: "ERR #{inspect(error)}"

  def tooling_basename(version, otp_major \\ System.otp_release()) do
    "agov-#{version}-#{@target_platform}-otp-#{otp_major}"
  end

  def bundle_basename(version), do: "agf-#{version}-bundle"

  defp read_version(path) do
    case File.read(path) do
      {:ok, content} -> {:ok, String.trim(content)}
      {:error, reason} -> {:error, ["could not read version file #{path}: #{inspect(reason)}"]}
    end
  end

  defp maybe_run_release(_app_dir, false), do: :ok

  defp maybe_run_release(app_dir, true) do
    {output, status} =
      System.cmd("mix", ["release", @release_name, "--overwrite"],
        cd: app_dir,
        env: [{"MIX_ENV", "prod"}],
        stderr_to_stdout: true
      )

    if status == 0 do
      :ok
    else
      {:error, ["mix release #{@release_name} failed:\n#{String.trim(output)}"]}
    end
  end

  defp resolve_release_dir(_app_dir, release_dir) when is_binary(release_dir) do
    if File.dir?(release_dir) do
      {:ok, release_dir}
    else
      {:error, ["release directory not found: #{release_dir}"]}
    end
  end

  defp resolve_release_dir(app_dir, nil) do
    release_dir = Path.join(app_dir, "_build/prod/rel/#{@release_name}")
    resolve_release_dir(app_dir, release_dir)
  end

  defp write_tooling_archive(release_dir, output_dir, version) do
    basename = tooling_basename(version)
    archive_path = Path.join(output_dir, basename <> ".tar.gz")

    with {:ok, stage_parent} <- stage_parent("agov_tooling"),
         stage_dir = Path.join(stage_parent, basename),
         {:ok, _copied} <- File.cp_r(release_dir, stage_dir),
         :ok <- assert_no_forbidden_entries(stage_dir),
         :ok <- tar_dir(stage_parent, basename, archive_path) do
      File.rm_rf(stage_parent)
      {:ok, archive_path}
    else
      {:error, reason} = error ->
        File.rm_rf(Path.dirname(archive_path) |> Path.join(".agov_tooling_stage"))
        if is_list(reason), do: error, else: {:error, [inspect(reason)]}
    end
  end

  defp write_tooling_provenance(output_dir, archive_path, checksum, version) do
    basename = tooling_basename(version)
    path = Path.join(output_dir, basename <> "-provenance.json")

    provenance = %{
      "version" => version,
      "source_commit" => System.get_env("CI_COMMIT_SHA", "unknown"),
      "tag" => System.get_env("CI_COMMIT_TAG") || "untagged",
      "build_timestamp" => DateTime.utc_now() |> DateTime.to_iso8601(),
      "pipeline_identity" => %{
        "pipeline_id" => System.get_env("CI_PIPELINE_ID", "unknown"),
        "job_id" => System.get_env("CI_JOB_ID", "unknown")
      },
      "archive_filename" => Path.basename(archive_path),
      "checksum" => checksum,
      "otp_version" => System.otp_release(),
      "elixir_version" => System.version(),
      "target_platform" => @target_platform,
      "packaging" => "mix-release"
    }

    with {:ok, json} <- Jason.encode(provenance, pretty: true),
         :ok <- File.write(path, json <> "\n") do
      {:ok, path}
    else
      {:error, reason} -> {:error, ["could not write tooling provenance: #{inspect(reason)}"]}
    end
  end

  defp bundle_inputs(output_dir, version) do
    expected = [
      Path.join(output_dir, "agf-#{version}.tar.gz"),
      Path.join(output_dir, "agf-#{version}.zip")
    ]

    tooling_glob = Path.join(output_dir, "agov-#{version}-#{@target_platform}-otp-*.tar.gz")

    tooling =
      tooling_glob
      |> Path.wildcard()
      |> Enum.reject(&String.ends_with?(&1, "-bundle.tar.gz"))
      |> Enum.sort()

    missing =
      expected
      |> Enum.reject(&File.regular?/1)
      |> Enum.map(&"missing bundle input: #{Path.basename(&1)}")

    missing =
      if tooling == [] do
        ["missing bundle input: agov-#{version}-#{@target_platform}-otp-*.tar.gz" | missing]
      else
        missing
      end

    case missing do
      [] -> {:ok, expected ++ [List.last(tooling)]}
      errors -> {:error, Enum.reverse(errors)}
    end
  end

  defp write_bundle_archive(output_dir, version, inputs) do
    basename = bundle_basename(version)
    archive_path = Path.join(output_dir, basename <> ".tar.gz")

    with {:ok, stage_parent} <- stage_parent("agf_bundle"),
         stage_dir = Path.join(stage_parent, basename),
         :ok <- File.mkdir_p(stage_dir),
         :ok <- copy_bundle_inputs(inputs, stage_dir),
         :ok <- write_bundle_sums(inputs, stage_dir),
         :ok <- write_bundle_provenance(inputs, stage_dir, version),
         :ok <- tar_dir(stage_parent, basename, archive_path) do
      File.rm_rf(stage_parent)
      {:ok, archive_path}
    else
      {:error, reason} = error ->
        File.rm_rf(Path.dirname(archive_path) |> Path.join(".agf_bundle_stage"))
        if is_list(reason), do: error, else: {:error, [inspect(reason)]}
    end
  end

  defp copy_bundle_inputs(inputs, stage_dir) do
    Enum.reduce_while(inputs, :ok, fn path, :ok ->
      case File.cp(path, Path.join(stage_dir, Path.basename(path))) do
        :ok -> {:cont, :ok}
        {:error, reason} -> {:halt, {:error, ["could not copy #{path}: #{inspect(reason)}"]}}
      end
    end)
  end

  defp write_bundle_sums(inputs, stage_dir) do
    with {:ok, lines} <- checksum_lines(inputs) do
      File.write(Path.join(stage_dir, "SHA256SUMS"), Enum.join(lines, ""))
    else
      {:error, reason} -> {:error, ["could not write bundle SHA256SUMS: #{inspect(reason)}"]}
    end
  end

  defp write_bundle_provenance(inputs, stage_dir, version) do
    with {:ok, checksums} <- checksums_by_basename(inputs),
         {:ok, json} <-
           Jason.encode(
             %{
               "version" => version,
               "source_commit" => System.get_env("CI_COMMIT_SHA", "unknown"),
               "tag" => System.get_env("CI_COMMIT_TAG") || "untagged",
               "build_timestamp" => DateTime.utc_now() |> DateTime.to_iso8601(),
               "pipeline_identity" => %{
                 "pipeline_id" => System.get_env("CI_PIPELINE_ID", "unknown"),
                 "job_id" => System.get_env("CI_JOB_ID", "unknown")
               },
               "archive_filenames" => Enum.map(inputs, &Path.basename/1),
               "checksums" => checksums
             },
             pretty: true
           ) do
      File.write(Path.join(stage_dir, "bundle-provenance.json"), json <> "\n")
    else
      {:error, reason} -> {:error, ["could not write bundle provenance: #{inspect(reason)}"]}
    end
  end

  defp checksum_lines(paths) do
    Enum.reduce_while(paths, {:ok, []}, fn path, {:ok, lines} ->
      case sha256_file(path) do
        {:ok, hex} -> {:cont, {:ok, ["#{hex}  #{Path.basename(path)}\n" | lines]}}
        {:error, reason} -> {:halt, {:error, reason}}
      end
    end)
    |> case do
      {:ok, lines} -> {:ok, Enum.reverse(lines)}
      error -> error
    end
  end

  defp checksums_by_basename(paths) do
    Enum.reduce_while(paths, {:ok, %{}}, fn path, {:ok, acc} ->
      case sha256_file(path) do
        {:ok, hex} -> {:cont, {:ok, Map.put(acc, Path.basename(path), hex)}}
        {:error, reason} -> {:halt, {:error, reason}}
      end
    end)
  end

  defp write_checksum(path) do
    checksum_path = checksum_path(path)

    with {:ok, hex} <- sha256_file(path),
         :ok <- File.write(checksum_path, "#{hex}  #{Path.basename(path)}\n") do
      {:ok, hex, checksum_path}
    else
      {:error, reason} -> {:error, ["could not write checksum for #{path}: #{inspect(reason)}"]}
    end
  end

  defp checksum_path(path) do
    cond do
      String.ends_with?(path, ".tar.gz") -> String.replace_suffix(path, ".tar.gz", ".sha256")
      true -> path <> ".sha256"
    end
  end

  defp sha256_file(path) do
    case File.read(path) do
      {:ok, content} -> {:ok, :crypto.hash(:sha256, content) |> Base.encode16(case: :lower)}
      {:error, reason} -> {:error, reason}
    end
  end

  defp tar_dir(stage_parent, basename, archive_path) do
    entries =
      stage_parent
      |> Path.join("#{basename}/**/*")
      |> Path.wildcard(match_dot: true)
      |> Enum.filter(&File.regular?/1)
      |> Enum.map(&Path.relative_to(&1, stage_parent))

    tar_name = Path.basename(archive_path)
    staged_archive = Path.join(stage_parent, tar_name)
    File.rm(archive_path)
    File.rm(staged_archive)

    result =
      File.cd!(stage_parent, fn ->
        tar_name
        |> String.to_charlist()
        |> :erl_tar.create(Enum.map(entries, &String.to_charlist/1), [:compressed])
      end)

    case result do
      :ok ->
        File.mkdir_p!(Path.dirname(archive_path))
        File.cp(staged_archive, archive_path)

      {:error, reason} ->
        {:error, ["could not write archive #{archive_path}: #{inspect(reason)}"]}
    end
  end

  defp assert_no_forbidden_entries(stage_dir) do
    forbidden = ["agf/", ".env", ".git/", "tmp/", "agents-govern/"]

    offenders =
      stage_dir
      |> Path.join("**/*")
      |> Path.wildcard(match_dot: true)
      |> Enum.map(&Path.relative_to(&1, stage_dir))
      |> Enum.filter(fn path ->
        Enum.any?(forbidden, fn
          ".env" ->
            path == ".env" || String.ends_with?(path, "/.env")

          pattern ->
            path == String.trim_trailing(pattern, "/") || String.starts_with?(path, pattern)
        end)
      end)

    case offenders do
      [] -> :ok
      paths -> {:error, Enum.map(paths, &"forbidden file selected for tooling archive: #{&1}")}
    end
  end

  defp stage_parent(name) do
    path = Path.join(System.tmp_dir!(), "#{name}_#{System.unique_integer([:positive])}")

    with {:ok, _removed} <- File.rm_rf(path),
         :ok <- File.mkdir_p(path) do
      {:ok, path}
    end
  end
end
