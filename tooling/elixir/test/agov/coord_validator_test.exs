defmodule Agov.CoordValidatorTest do
  use ExUnit.Case, async: true

  alias Agov.CoordValidator

  @fixtures_dir Path.join([__DIR__, "..", "fixtures"])

  defp fixture(name), do: Path.join(@fixtures_dir, name)

  describe "validate_file/1" do
    test "returns :ok for a valid claim message" do
      assert {:ok, _data} = CoordValidator.validate_file(fixture("valid_claim.md"))
    end

    test "returns :ok for a valid ack message" do
      assert {:ok, _data} = CoordValidator.validate_file(fixture("valid_ack.md"))
    end

    test "returns {:error, errors} list for message missing required field kind" do
      assert {:error, errors} = CoordValidator.validate_file(fixture("invalid_missing_kind.md"))
      assert is_list(errors)
      assert length(errors) > 0
    end

    test "returns {:error, errors} for claim with empty artifacts_owned" do
      assert {:error, errors} = CoordValidator.validate_file(fixture("invalid_claim_no_artifacts.md"))
      assert is_list(errors)
      assert length(errors) > 0
    end

    test "returns {:error, :no_frontmatter} for file with no YAML front matter" do
      assert {:error, :no_frontmatter} = CoordValidator.validate_file(fixture("no_frontmatter.md"))
    end

    test "returns {:error, :file_not_found} for a missing file" do
      assert {:error, :file_not_found} = CoordValidator.validate_file("/nonexistent/path.md")
    end
  end

  describe "advisory_findings/1" do
    test "returns empty list when model field is present" do
      data = %{"model" => "claude-sonnet-4-6", "kind" => "note"}
      assert CoordValidator.advisory_findings(data) == []
    end

    test "returns advisory warning when model field is absent" do
      data = %{"kind" => "note", "from" => "planner-architect"}
      advisories = CoordValidator.advisory_findings(data)
      assert length(advisories) == 1
      assert hd(advisories) =~ "model"
    end

    test "returns no advisory for file with model field" do
      assert {:ok, data} = CoordValidator.validate_file(fixture("valid_note_with_model.md"))
      assert CoordValidator.advisory_findings(data) == []
    end

    test "returns advisory warning for file without model field" do
      assert {:ok, data} = CoordValidator.validate_file(fixture("valid_note_no_model.md"))
      advisories = CoordValidator.advisory_findings(data)
      assert length(advisories) == 1
      assert hd(advisories) =~ "model"
    end
  end

  describe "extract_frontmatter/1" do
    test "extracts YAML block from file with front matter" do
      content = "---\nkind: note\nfrom: claude\n---\nBody text"
      assert {:ok, yaml_str} = CoordValidator.extract_frontmatter(content)
      assert String.contains?(yaml_str, "kind: note")
    end

    test "returns :no_frontmatter for content without --- delimiters" do
      assert {:error, :no_frontmatter} = CoordValidator.extract_frontmatter("No front matter here")
    end
  end
end
