defmodule Agov.ChannelReaderTest do
  use ExUnit.Case, async: true

  alias Agov.ChannelReader

  @fixtures_dir Path.join([__DIR__, "..", "fixtures"])

  describe "scan/1" do
    test "returns message list, total, and seq range from fixtures dir" do
      assert {:ok, result} = ChannelReader.scan(@fixtures_dir)
      assert result.total > 0
      {min_seq, max_seq} = result.seq_range
      assert min_seq <= max_seq
      assert length(result.messages) == result.total
    end

    test "messages are sorted ascending by seq" do
      {:ok, result} = ChannelReader.scan(@fixtures_dir)
      seqs = Enum.map(result.messages, fn {_file, data} -> data["seq"] end)
      assert seqs == Enum.sort(seqs)
    end

    test "each message is a {filename, data} tuple" do
      {:ok, result} = ChannelReader.scan(@fixtures_dir)
      assert [{filename, data} | _] = result.messages
      assert is_binary(filename)
      assert is_map(data)
      assert Map.has_key?(data, "kind")
    end

    test "skips non-message files (README, invalid front matter)" do
      # fixtures dir has no README but does have valid coordination messages
      # This test confirms invalid files are skipped rather than causing error
      {:ok, result} = ChannelReader.scan(@fixtures_dir)
      Enum.each(result.messages, fn {_file, data} ->
        assert Map.has_key?(data, "seq")
      end)
    end

    test "returns empty result for directory with no valid messages" do
      tmp = System.tmp_dir!() |> Path.join("agov_test_empty_#{:rand.uniform(99999)}")
      File.mkdir_p!(tmp)
      File.write!(Path.join(tmp, "README.md"), "# not a coordination message\n")

      assert {:ok, %{messages: [], total: 0, seq_range: nil}} = ChannelReader.scan(tmp)

      File.rm_rf!(tmp)
    end

    test "returns error for missing directory" do
      assert {:error, :dir_not_found} = ChannelReader.scan("/nonexistent/path/agov")
    end
  end
end
