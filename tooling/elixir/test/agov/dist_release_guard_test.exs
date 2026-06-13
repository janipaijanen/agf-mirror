defmodule Agov.DistReleaseGuardTest do
  use ExUnit.Case, async: false

  alias Agov.DistReleaseGuard

  describe "check/2" do
    test "accepts tag with v-prefix matching VERSION" do
      assert :ok = DistReleaseGuard.check("v2.0.0", "2.0.0")
    end

    test "accepts tag without v-prefix matching VERSION" do
      assert :ok = DistReleaseGuard.check("2.0.0", "2.0.0")
    end

    # gate-self-test-rule: negative test — guard fires when tag does not match VERSION
    test "rejects tag that does not match VERSION" do
      assert {:error, msg} = DistReleaseGuard.check("v9.99.99", "2.0.0")
      assert msg =~ "v9.99.99"
      assert msg =~ "2.0.0"
    end

    test "returns :no_tag when tag is nil (local build)" do
      assert :no_tag = DistReleaseGuard.check(nil, "2.0.0")
    end

    test "returns :no_tag when tag is empty string" do
      assert :no_tag = DistReleaseGuard.check("", "2.0.0")
    end
  end

  describe "check_from_env/1" do
    test "reads CI_COMMIT_TAG from environment and accepts matching version" do
      version_path = write_version!("2.0.0")
      System.put_env("CI_COMMIT_TAG", "v2.0.0")
      assert :ok = DistReleaseGuard.check_from_env(version_path)
    after
      System.delete_env("CI_COMMIT_TAG")
    end

    test "returns :no_tag when CI_COMMIT_TAG is not set" do
      version_path = write_version!("2.0.0")
      System.delete_env("CI_COMMIT_TAG")
      assert :no_tag = DistReleaseGuard.check_from_env(version_path)
    end

    # gate-self-test-rule: negative test via env — guard fires when env tag mismatches file
    test "returns error when CI_COMMIT_TAG does not match version file" do
      version_path = write_version!("2.0.0")
      System.put_env("CI_COMMIT_TAG", "v9.99.99")
      assert {:error, msg} = DistReleaseGuard.check_from_env(version_path)
      assert msg =~ "v9.99.99"
      assert msg =~ "2.0.0"
    after
      System.delete_env("CI_COMMIT_TAG")
    end

    test "returns error when version file does not exist" do
      System.put_env("CI_COMMIT_TAG", "v2.0.0")
      assert {:error, msg} = DistReleaseGuard.check_from_env("/nonexistent/VERSION")
      assert msg =~ "VERSION"
    after
      System.delete_env("CI_COMMIT_TAG")
    end
  end

  defp write_version!(content) do
    path = Path.join(System.tmp_dir!(), "VERSION_#{System.unique_integer([:positive])}")
    File.write!(path, content <> "\n")
    path
  end
end
