defmodule Agov.DistManifestTest do
  use ExUnit.Case, async: false

  alias Agov.DistManifest

  test "validates the repository release manifest" do
    assert {:ok, spec} = DistManifest.validate("../../agf/dist/release-manifest.yaml")

    expected_version = "../../agf/VERSION" |> File.read!() |> String.trim()
    assert spec.package["name"] == "agf"
    assert spec.version == expected_version
    assert "agf/VERSION" in spec.files
    assert "agf/dist/release-manifest.yaml" in spec.files
    refute Enum.any?(spec.files, &String.starts_with?(&1, "agents-govern/"))
    refute Enum.any?(spec.files, &String.starts_with?(&1, "tmp/"))
  end

  test "rejects forbidden selected files" do
    root = tmp_root!()
    write!(root, "agf/VERSION", "2.0.0\n")
    write!(root, "tmp/leak.txt", "nope\n")

    manifest =
      write!(root, "manifest.yaml", """
      schema_version: 1
      package:
        name: agf
        version_file: agf/VERSION
        output_dir: dist
        basename: agf-{{version}}
        formats: [tar.gz]
      include:
        - glob: tmp/**
      forbidden:
        - tmp/**
      """)

    assert {:error, errors} = DistManifest.validate(manifest, root: root)
    assert Enum.any?(errors, &(&1 == "forbidden file selected for release: tmp/leak.txt"))
  end

  test "build writes tar.gz and zip archives with manifest-selected files" do
    root = tmp_root!()
    write!(root, "agf/VERSION", "2.0.0\n")
    write!(root, "agf/LICENSE", "license\n")

    manifest =
      write!(root, "agf/dist/release-manifest.yaml", """
      schema_version: 1
      package:
        name: agf
        version_file: agf/VERSION
        output_dir: dist
        basename: agf-{{version}}
        formats:
          - tar.gz
          - zip
      include:
        - path: agf/VERSION
        - path: agf/LICENSE
      forbidden:
        - tmp/**
      """)

    assert {:ok, spec} = DistManifest.build(manifest, root: root)

    tar_path = spec.outputs["tar.gz"]
    zip_path = spec.outputs["zip"]

    assert File.regular?(tar_path)
    assert File.regular?(zip_path)

    assert {:ok, tar_files} = :erl_tar.table(String.to_charlist(tar_path), [:compressed])
    assert Enum.sort(Enum.map(tar_files, &to_string/1)) == ["agf/LICENSE", "agf/VERSION"]

    assert {:ok, zip_files} = :zip.list_dir(String.to_charlist(zip_path))

    zip_names =
      zip_files
      |> Enum.filter(&match?({:zip_file, _name, _info, _comment, _offset, _comp_size}, &1))
      |> Enum.map(fn {:zip_file, name, _info, _comment, _offset, _comp_size} ->
        to_string(name)
      end)
      |> Enum.sort()

    assert zip_names == ["agf/LICENSE", "agf/VERSION"]
  end

  test "build writes sha256 checksum file beside each archive" do
    root = tmp_root!()
    write!(root, "agf/VERSION", "2.0.0\n")
    write!(root, "agf/LICENSE", "license\n")

    manifest =
      write!(root, "agf/dist/release-manifest.yaml", """
      schema_version: 1
      package:
        name: agf
        version_file: agf/VERSION
        output_dir: dist
        basename: agf-{{version}}
        formats:
          - tar.gz
          - zip
      include:
        - path: agf/VERSION
        - path: agf/LICENSE
      forbidden:
        - tmp/**
      """)

    assert {:ok, spec} = DistManifest.build(manifest, root: root)

    tar_sha256_path = spec.outputs["tar.gz"] <> ".sha256"
    zip_sha256_path = spec.outputs["zip"] <> ".sha256"

    assert File.regular?(tar_sha256_path), "expected #{tar_sha256_path} to exist"
    assert File.regular?(zip_sha256_path), "expected #{zip_sha256_path} to exist"

    # format: "<64-hex-chars>  <basename>\n"
    tar_content = File.read!(tar_sha256_path)
    assert tar_content =~ ~r/^[0-9a-f]{64}  agf-2\.0\.0\.tar\.gz\n$/
    zip_content = File.read!(zip_sha256_path)
    assert zip_content =~ ~r/^[0-9a-f]{64}  agf-2\.0\.0\.zip\n$/

    # checksums are also present in the returned spec
    assert Map.has_key?(spec, :checksums)
    assert byte_size(spec.checksums["tar.gz"]) == 64
    assert byte_size(spec.checksums["zip"]) == 64
  end

  test "build writes provenance JSON with required fields" do
    root = tmp_root!()
    write!(root, "agf/VERSION", "2.0.0\n")
    write!(root, "agf/LICENSE", "license\n")

    manifest =
      write!(root, "agf/dist/release-manifest.yaml", """
      schema_version: 1
      package:
        name: agf
        version_file: agf/VERSION
        output_dir: dist
        basename: agf-{{version}}
        formats:
          - tar.gz
          - zip
      include:
        - path: agf/VERSION
        - path: agf/LICENSE
      forbidden:
        - tmp/**
      """)

    assert {:ok, spec} = DistManifest.build(manifest, root: root)

    provenance_path = spec.provenance_path
    assert provenance_path != nil
    assert File.regular?(provenance_path), "expected #{provenance_path} to exist"

    prov = Jason.decode!(File.read!(provenance_path))

    assert prov["version"] == "2.0.0"
    assert is_binary(prov["source_commit"])
    assert is_binary(prov["tag"])
    assert is_binary(prov["build_timestamp"])
    assert is_map(prov["pipeline_identity"])
    assert is_binary(prov["manifest_path"])
    assert is_list(prov["archive_filenames"])
    assert length(prov["archive_filenames"]) == 2
    assert is_map(prov["checksums"])
    assert Map.has_key?(prov["checksums"], "agf-2.0.0.tar.gz")
    assert Map.has_key?(prov["checksums"], "agf-2.0.0.zip")
  end

  defp tmp_root! do
    path =
      Path.join(System.tmp_dir!(), "agov_dist_manifest_#{System.unique_integer([:positive])}")

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
