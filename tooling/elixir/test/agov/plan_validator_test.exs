defmodule Agov.PlanValidatorTest do
  use ExUnit.Case, async: false

  alias Agov.PlanValidator

  @fixtures_dir Path.join([__DIR__, "..", "fixtures"])

  defp fixture(name), do: Path.join(@fixtures_dir, name)

  setup do
    previous = System.get_env("AGOV_MANIFEST_PATH")

    on_exit(fn ->
      if previous do
        System.put_env("AGOV_MANIFEST_PATH", previous)
      else
        System.delete_env("AGOV_MANIFEST_PATH")
      end
    end)
  end

  describe "validate_file/1" do
    test "returns :ok for a valid plan" do
      assert {:ok, data} = PlanValidator.validate_file(fixture("valid_plan.md"))
      assert data["id"] == "valid-plan"
    end

    test "returns {:error, errors} for missing advisory_consultation" do
      assert {:error, errors} =
               PlanValidator.validate_file(fixture("invalid_plan_no_advisory.md"))

      assert is_list(errors)
      assert length(errors) > 0
    end

    test "returns {:error, errors} for waived role without reason" do
      assert {:error, errors} =
               PlanValidator.validate_file(fixture("invalid_plan_waived_no_reason.md"))

      assert is_list(errors)
      assert length(errors) > 0
    end

    test "returns {:error, errors} when infosec-sentinel is missing" do
      assert {:error, errors} =
               PlanValidator.validate_file(fixture("invalid_plan_missing_infosec.md"))

      assert {"missing required advisory role: infosec-sentinel", "#/advisory_consultation"} in errors
    end

    test "returns {:error, errors} when contribution-auditor is missing" do
      assert {:error, errors} =
               PlanValidator.validate_file(fixture("invalid_plan_missing_auditor.md"))

      assert {"missing required advisory role: contribution-auditor", "#/advisory_consultation"} in errors
    end

    test "returns {:error, errors} for duplicate advisory role entries" do
      assert {:error, errors} =
               PlanValidator.validate_file(fixture("invalid_plan_duplicate_role.md"))

      assert {"duplicate advisory role: infosec-sentinel", "#/advisory_consultation"} in errors
    end

    test "returns {:error, errors} for unknown advisory roles" do
      assert {:error, errors} =
               PlanValidator.validate_file(fixture("invalid_plan_unknown_role.md"))

      assert {"unknown advisory role: made-up-advisor", "#/advisory_consultation"} in errors
    end

    test "allows optional advisory roles when they are active in the manifest" do
      System.put_env("AGOV_MANIFEST_PATH", fixture("manifest_with_optional_advisors.yaml"))

      assert {:ok, data} =
               PlanValidator.validate_file(fixture("valid_plan_optional_active_advisors.md"))

      roles = Enum.map(data["advisory_consultation"], &Map.fetch!(&1, "role"))

      assert "fact-checker" in roles
      assert "academic-peer-reviewer" in roles
    end

    test "rejects optional advisory roles when they are not active in the manifest" do
      System.put_env("AGOV_MANIFEST_PATH", fixture("manifest_with_optional_advisors.yaml"))

      assert {:error, errors} =
               PlanValidator.validate_file(fixture("invalid_plan_suspended_advisor.md"))

      assert {"unknown advisory role: capability-developer", "#/advisory_consultation"} in errors
    end

    test "returns {:error, :no_frontmatter} for file with no YAML front matter" do
      assert {:error, :no_frontmatter} =
               PlanValidator.validate_file(fixture("no_frontmatter.md"))
    end

    test "returns {:error, :file_not_found} for a missing file" do
      assert {:error, :file_not_found} = PlanValidator.validate_file("/nonexistent/path.md")
    end
  end
end
