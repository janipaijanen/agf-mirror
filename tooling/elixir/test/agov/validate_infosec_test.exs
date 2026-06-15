defmodule Agov.ValidateInfosecTest do
  use ExUnit.Case, async: true

  test "accepts a clean YAML file" do
    dir = tmp_dir("infosec-clean")
    path = Path.join(dir, "clean.yaml")
    File.write!(path, "project:\n  codename: fixture\n")

    assert {:ok, []} = Agov.ValidateInfosec.validate_file(path)
  end

  test "rejects token-like material" do
    dir = tmp_dir("infosec-token")
    path = Path.join(dir, "leaky.yaml")
    File.write!(path, "secret: glpat-abcdefghijklmnop\n")

    assert {:error, findings} = Agov.ValidateInfosec.validate_file(path)
    assert Enum.any?(findings, &(&1.message == "VCS token detected"))
  end

  defp tmp_dir(name) do
    path = Path.join(System.tmp_dir!(), "agov-#{name}-#{System.unique_integer([:positive])}")
    File.mkdir_p!(path)
    ExUnit.Callbacks.on_exit(fn -> File.rm_rf!(path) end)
    path
  end
end
