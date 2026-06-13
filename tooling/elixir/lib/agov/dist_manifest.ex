defmodule Agov.DistManifest do
  @moduledoc false

  @default_manifest "../../agf/dist/release-manifest.yaml"
  @default_root "../.."

  def default_manifest_path, do: @default_manifest
  def default_root, do: @default_root

  def validate(manifest_path \\ @default_manifest, opts \\ []) do
    root = opts |> Keyword.get(:root, @default_root) |> Path.expand()
    manifest_path = Path.expand(manifest_path)

    with {:ok, manifest} <- load_manifest(manifest_path),
         :ok <- validate_schema_version(manifest),
         {:ok, package} <- package(manifest),
         {:ok, version} <- version(root, package),
         {:ok, files} <- files(root, manifest),
         :ok <- validate_outputs(package) do
      {:ok,
       %{
         manifest_path: manifest_path,
         root: root,
         package: package,
         version: version,
         files: files,
         outputs: output_paths(root, package, version)
       }}
    end
  end

  def build(manifest_path \\ @default_manifest, opts \\ []) do
    with {:ok, spec} <- validate(manifest_path, opts),
         :ok <- File.mkdir_p(Path.join(spec.root, spec.package["output_dir"] || "dist")),
         :ok <- write_archives(spec) do
      {:ok, spec}
    end
  end

  def format_summary({:ok, spec}) do
    outputs =
      spec.outputs
      |> Enum.map(fn {_format, path} -> "  - #{Path.relative_to(path, spec.root)}" end)
      |> Enum.join("\n")

    """
    OK  release manifest valid
    Manifest: #{Path.relative_to(spec.manifest_path, spec.root)}
    Package:  #{spec.package["name"]} #{spec.version}
    Files:    #{length(spec.files)}
    Outputs:
    #{outputs}
    """
    |> String.trim_trailing()
  end

  def format_summary({:error, errors}) when is_list(errors) do
    lines = Enum.map(errors, &"ERR #{&1}")
    Enum.join(lines, "\n")
  end

  def format_summary({:error, error}), do: "ERR #{inspect(error)}"

  defp load_manifest(path) do
    case File.read(path) do
      {:ok, content} ->
        case YamlElixir.read_from_string(content) do
          {:ok, data} when is_map(data) -> {:ok, data}
          {:ok, _} -> {:error, ["manifest must be a YAML mapping"]}
          {:error, reason} -> {:error, ["could not parse manifest YAML: #{inspect(reason)}"]}
        end

      {:error, :enoent} ->
        {:error, ["manifest not found: #{path}"]}

      {:error, reason} ->
        {:error, ["could not read manifest #{path}: #{inspect(reason)}"]}
    end
  end

  defp validate_schema_version(%{"schema_version" => 1}), do: :ok

  defp validate_schema_version(_manifest) do
    {:error, ["schema_version must be 1"]}
  end

  defp package(%{"package" => package}) when is_map(package) do
    required = ~w(name version_file output_dir basename formats)
    missing = Enum.reject(required, &Map.has_key?(package, &1))

    if missing == [] do
      {:ok, package}
    else
      {:error, Enum.map(missing, &"package.#{&1} is required")}
    end
  end

  defp package(_manifest), do: {:error, ["package mapping is required"]}

  defp version(root, package) do
    path = safe_join(root, package["version_file"])

    case path && File.read(path) do
      {:ok, content} -> {:ok, String.trim(content)}
      {:error, reason} -> {:error, ["could not read package.version_file: #{inspect(reason)}"]}
      nil -> {:error, ["package.version_file must be a relative path inside the repository"]}
    end
  end

  defp files(root, manifest) do
    includes = Map.get(manifest, "include", [])
    excludes = Map.get(manifest, "exclude", [])
    forbidden = Map.get(manifest, "forbidden", [])

    with :ok <- validate_list("include", includes),
         :ok <- validate_list("exclude", excludes),
         :ok <- validate_list("forbidden", forbidden),
         {:ok, included_files} <- expand_includes(root, includes) do
      excluded? = matcher(root, excludes)
      forbidden? = matcher(root, forbidden)

      files =
        included_files
        |> Enum.reject(excluded?)
        |> Enum.sort()
        |> Enum.uniq()

      errors =
        []
        |> Kernel.++(forbidden_errors(files, forbidden?))
        |> Kernel.++(duplicate_archive_errors(files))

      case errors do
        [] -> {:ok, files}
        _ -> {:error, errors}
      end
    end
  end

  defp validate_list(_name, value) when is_list(value), do: :ok
  defp validate_list(name, _value), do: {:error, ["#{name} must be a list"]}

  defp expand_includes(root, includes) do
    {files, errors} =
      Enum.reduce(includes, {[], []}, fn entry, {files, errors} ->
        case expand_include(root, entry) do
          {:ok, more} -> {more ++ files, errors}
          {:error, error} -> {files, [error | errors]}
        end
      end)

    case Enum.reverse(errors) do
      [] -> {:ok, files}
      errors -> {:error, errors}
    end
  end

  defp expand_include(root, path) when is_binary(path) do
    expand_include(root, %{"path" => path})
  end

  defp expand_include(root, %{"path" => path} = entry) when is_binary(path) do
    optional = truthy?(Map.get(entry, "optional"))

    case safe_join(root, path) do
      nil ->
        {:error, "include path escapes repository: #{path}"}

      full_path ->
        cond do
          File.regular?(full_path) -> {:ok, [path]}
          optional -> {:ok, []}
          true -> {:error, "include path not found or not a file: #{path}"}
        end
    end
  end

  defp expand_include(root, %{"glob" => glob} = entry) when is_binary(glob) do
    optional = truthy?(Map.get(entry, "optional"))

    case safe_join(root, glob) do
      nil ->
        {:error, "include glob escapes repository: #{glob}"}

      full_glob ->
        files =
          full_glob
          |> Path.wildcard(match_dot: true)
          |> Enum.filter(&File.regular?/1)
          |> Enum.map(&Path.relative_to(&1, root))
          |> Enum.reject(&String.starts_with?(&1, "../"))

        cond do
          files != [] -> {:ok, files}
          optional -> {:ok, []}
          true -> {:error, "include glob matched no files: #{glob}"}
        end
    end
  end

  defp expand_include(_root, entry), do: {:error, "invalid include entry: #{inspect(entry)}"}

  defp matcher(root, patterns) do
    compiled =
      Enum.map(patterns, fn pattern ->
        pattern
        |> safe_pattern(root)
        |> Path.wildcard(match_dot: true)
        |> Enum.map(&Path.relative_to(&1, root))
        |> MapSet.new()
        |> then(&{pattern, &1})
      end)

    fn path ->
      Enum.any?(compiled, fn {pattern, matches} ->
        MapSet.member?(matches, path) || glob_prefix_match?(pattern, path)
      end)
    end
  end

  defp safe_pattern(root, pattern) when is_binary(pattern) do
    safe_join(root, pattern) || Path.join(root, "__invalid__")
  end

  defp glob_prefix_match?(pattern, path) do
    case String.split(pattern, "/**", parts: 2) do
      [prefix, _] -> path == prefix || String.starts_with?(path, prefix <> "/")
      _ -> false
    end
  end

  defp forbidden_errors(files, forbidden?) do
    files
    |> Enum.filter(forbidden?)
    |> Enum.map(&"forbidden file selected for release: #{&1}")
  end

  defp duplicate_archive_errors(files) do
    files
    |> Enum.frequencies_by(&archive_path/1)
    |> Enum.filter(fn {_path, count} -> count > 1 end)
    |> Enum.map(fn {path, _count} -> "duplicate archive path: #{path}" end)
  end

  defp validate_outputs(package) do
    formats = package["formats"]

    cond do
      not is_list(formats) ->
        {:error, ["package.formats must be a list"]}

      Enum.any?(formats, &(&1 not in ["tar.gz", "zip"])) ->
        {:error, ["package.formats may only contain tar.gz and zip"]}

      true ->
        :ok
    end
  end

  defp output_paths(root, package, version) do
    output_dir = Path.join(root, package["output_dir"])
    basename = String.replace(package["basename"], "{{version}}", version)

    Map.new(package["formats"], fn
      "tar.gz" -> {"tar.gz", Path.join(output_dir, basename <> ".tar.gz")}
      "zip" -> {"zip", Path.join(output_dir, basename <> ".zip")}
    end)
  end

  defp write_archives(spec) do
    tar_result =
      if Map.has_key?(spec.outputs, "tar.gz") do
        write_tar(spec.outputs["tar.gz"], spec.root, spec.files)
      else
        :ok
      end

    zip_result =
      if Map.has_key?(spec.outputs, "zip") do
        write_zip(spec.outputs["zip"], spec.root, spec.files)
      else
        :ok
      end

    with :ok <- tar_result, :ok <- zip_result, do: :ok
  end

  defp write_tar(output, root, files) do
    File.cd!(root, fn ->
      output
      |> String.to_charlist()
      |> :erl_tar.create(Enum.map(files, &String.to_charlist/1), [:compressed])
    end)
    |> case do
      :ok -> :ok
      {:error, reason} -> {:error, ["could not write tar.gz archive: #{inspect(reason)}"]}
    end
  end

  defp write_zip(output, root, files) do
    File.cd!(root, fn ->
      output
      |> String.to_charlist()
      |> :zip.create(Enum.map(files, &String.to_charlist/1))
    end)
    |> case do
      {:ok, _} -> :ok
      {:error, reason} -> {:error, ["could not write zip archive: #{inspect(reason)}"]}
    end
  end

  defp safe_join(root, path) when is_binary(path) do
    cond do
      Path.type(path) == :absolute ->
        nil

      String.split(path, "/") |> Enum.member?("..") ->
        nil

      true ->
        full = Path.expand(path, root)

        if full == root || String.starts_with?(full, root <> "/") do
          full
        end
    end
  end

  defp archive_path(path), do: path

  defp truthy?(value) when value in [true, "true", "yes", "1"], do: true
  defp truthy?(_value), do: false
end
