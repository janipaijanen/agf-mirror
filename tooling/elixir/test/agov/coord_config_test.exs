defmodule Agov.CoordConfigTest do
  use ExUnit.Case, async: true

  alias Agov.CoordConfig

  @tag :tmp_dir
  describe "load/1" do
    test "missing config file returns empty aliases" do
      result = CoordConfig.load("/nonexistent/path/agov_test_xyz")
      assert result == %{from_aliases: %{}}
    end

    test "valid config returns alias map" do
      tmp = System.tmp_dir!() |> Path.join("agov_config_test_#{:rand.uniform(99999)}")
      File.mkdir_p!(tmp)

      File.write!(Path.join(tmp, "agov-config.yaml"), """
      from_aliases:
        claude: planner-architect
        old-bot: coder-tester
      """)

      result = CoordConfig.load(tmp)
      assert result.from_aliases == %{"claude" => "planner-architect", "old-bot" => "coder-tester"}

      File.rm_rf!(tmp)
    end

    test "config with no from_aliases key returns empty" do
      tmp = System.tmp_dir!() |> Path.join("agov_config_test_#{:rand.uniform(99999)}")
      File.mkdir_p!(tmp)
      File.write!(Path.join(tmp, "agov-config.yaml"), "other_key: value\n")

      result = CoordConfig.load(tmp)
      assert result == %{from_aliases: %{}}

      File.rm_rf!(tmp)
    end

    test "from_aliases with non-map value returns empty" do
      tmp = System.tmp_dir!() |> Path.join("agov_config_test_#{:rand.uniform(99999)}")
      File.mkdir_p!(tmp)
      File.write!(Path.join(tmp, "agov-config.yaml"), "from_aliases: not-a-map\n")

      result = CoordConfig.load(tmp)
      assert result == %{from_aliases: %{}}

      File.rm_rf!(tmp)
    end
  end
end
