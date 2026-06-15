defmodule Agov.ValidateFrameworkConfigTest do
  use ExUnit.Case, async: true

  test "skips absent framework.yaml" do
    root = tmp_dir("framework-absent")

    assert {:ok, :skipped} = Agov.ValidateFrameworkConfig.validate_default(root)
  end

  test "accepts framework config matching VERSION" do
    root = framework_root("2.0.0", "2.0.0")

    assert {:ok, _data} =
             Agov.ValidateFrameworkConfig.validate_file(Path.join(root, "framework.yaml"))
  end

  test "rejects framework config that drifts from VERSION" do
    root = framework_root("2.0.0", "2.1.0")

    assert {:error, errors} =
             Agov.ValidateFrameworkConfig.validate_file(Path.join(root, "framework.yaml"))

    assert Enum.any?(errors, &(&1.message == "contract_version does not match VERSION"))
  end

  defp framework_root(version, contract_version) do
    root = tmp_dir("framework")
    File.write!(Path.join(root, "VERSION"), version <> "\n")

    File.write!(Path.join(root, "framework.yaml"), """
    contract_version: #{contract_version}
    compliance_target: artefact-only
    """)

    root
  end

  defp tmp_dir(name) do
    path = Path.join(System.tmp_dir!(), "agov-#{name}-#{System.unique_integer([:positive])}")
    File.mkdir_p!(path)
    ExUnit.Callbacks.on_exit(fn -> File.rm_rf!(path) end)
    path
  end
end
