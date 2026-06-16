defmodule Agov.ReleasePackageTest do
  use ExUnit.Case, async: false
  import Bitwise

  alias Agov.ReleasePackage

  test "tooling build packages release tree, checksum, and provenance" do
    root = tmp_root!()
    release_dir = Path.join(root, "fake_rel/agov")

    write!(root, "agf/VERSION", "2.0.0\n")
    agov_bin = write!(release_dir, "bin/agov", File.read!("rel/overlays/bin/agov"))
    File.chmod!(agov_bin, 0o755)
    write!(release_dir, "releases/0.1.0/start.boot", "boot\n")

    assert {:ok, result} =
             ReleasePackage.build_tooling(
               root: root,
               release_dir: release_dir,
               run_release?: false
             )

    assert File.regular?(result.archive_path)
    assert File.regular?(result.checksum_path)
    assert File.regular?(result.provenance_path)

    basename = ReleasePackage.tooling_basename("2.0.0")
    assert Path.basename(result.archive_path) == basename <> ".tar.gz"

    assert {:ok, tar_files} =
             :erl_tar.table(String.to_charlist(result.archive_path), [:compressed])

    names = Enum.map(tar_files, &to_string/1)
    assert "#{basename}/bin/agov" in names
    assert "#{basename}/releases/0.1.0/start.boot" in names
    refute Enum.any?(names, &String.contains?(&1, "agents-govern"))
    refute Enum.any?(names, &String.contains?(&1, "/.env"))

    checksum = File.read!(result.checksum_path)
    assert checksum =~ ~r/^[0-9a-f]{64}  agov-2\.0\.0-linux-x86_64-otp-\d+\.tar\.gz\n$/

    provenance = result.provenance_path |> File.read!() |> Jason.decode!()
    assert provenance["version"] == "2.0.0"
    assert provenance["otp_version"] == System.otp_release()
    assert provenance["elixir_version"] == System.version()
    assert provenance["target_platform"] == "linux-x86_64"
    assert provenance["packaging"] == "mix-release"

    extract_dir = Path.join(root, "extract-tooling")
    File.mkdir_p!(extract_dir)

    assert :ok =
             :erl_tar.extract(String.to_charlist(result.archive_path), [
               :compressed,
               {:cwd, String.to_charlist(extract_dir)}
             ])

    agov_script = Path.join([extract_dir, basename, "bin/agov"])
    assert File.read!(agov_script) =~ "Elixir.Agov.CLI"

    mode = File.stat!(agov_script).mode
    assert (mode &&& 0o111) != 0
  end

  test "bundle build wraps framework and tooling archives with sums and provenance" do
    root = tmp_root!()
    write!(root, "agf/VERSION", "2.0.0\n")
    write!(root, "dist/agf-2.0.0.tar.gz", "framework tar\n")
    write!(root, "dist/agf-2.0.0.zip", "framework zip\n")
    write!(root, "dist/#{ReleasePackage.tooling_basename("2.0.0")}.tar.gz", "tooling tar\n")

    assert {:ok, result} = ReleasePackage.build_bundle(root: root)

    assert File.regular?(result.archive_path)
    assert File.regular?(result.checksum_path)
    assert Path.basename(result.archive_path) == "agf-2.0.0-bundle.tar.gz"

    assert {:ok, tar_files} =
             :erl_tar.table(String.to_charlist(result.archive_path), [:compressed])

    names = tar_files |> Enum.map(&to_string/1) |> Enum.sort()

    assert "agf-2.0.0-bundle/SHA256SUMS" in names
    assert "agf-2.0.0-bundle/bundle-provenance.json" in names
    assert "agf-2.0.0-bundle/agf-2.0.0.tar.gz" in names
    assert "agf-2.0.0-bundle/agf-2.0.0.zip" in names
    assert "agf-2.0.0-bundle/#{ReleasePackage.tooling_basename("2.0.0")}.tar.gz" in names

    extract_dir = Path.join(root, "extract")
    File.mkdir_p!(extract_dir)

    assert :ok =
             :erl_tar.extract(
               String.to_charlist(result.archive_path),
               [:compressed, {:cwd, String.to_charlist(extract_dir)}]
             )

    sums = File.read!(Path.join(root, "extract/agf-2.0.0-bundle/SHA256SUMS"))
    assert sums =~ "agf-2.0.0.tar.gz"
    assert sums =~ "agf-2.0.0.zip"
    assert sums =~ ReleasePackage.tooling_basename("2.0.0") <> ".tar.gz"

    provenance =
      root
      |> Path.join("extract/agf-2.0.0-bundle/bundle-provenance.json")
      |> File.read!()
      |> Jason.decode!()

    assert provenance["version"] == "2.0.0"
    assert length(provenance["archive_filenames"]) == 3
    assert is_map(provenance["checksums"])
  end

  test "bundle build fails cleanly when tooling archive is missing" do
    root = tmp_root!()
    write!(root, "agf/VERSION", "2.0.0\n")
    write!(root, "dist/agf-2.0.0.tar.gz", "framework tar\n")
    write!(root, "dist/agf-2.0.0.zip", "framework zip\n")

    assert {:error, errors} = ReleasePackage.build_bundle(root: root)
    assert Enum.any?(errors, &String.contains?(&1, "missing bundle input: agov-2.0.0"))
  end

  defp tmp_root! do
    path =
      Path.join(System.tmp_dir!(), "agov_release_package_#{System.unique_integer([:positive])}")

    File.rm_rf!(path)
    File.mkdir_p!(path)
    path
  end

  defp write!(root, path, content) do
    full_path = Path.join(root, path)
    full_path |> Path.dirname() |> File.mkdir_p!()
    File.write!(full_path, content)
    full_path
  end
end
