defmodule Agov.ChannelReaderTest do
  use ExUnit.Case, async: true

  alias Agov.ChannelReader

  @fixtures_dir Path.join([__DIR__, "..", "fixtures"])

  describe "scan/1" do
    test "returns message list, total, and timestamp range from fixtures dir" do
      assert {:ok, result} = ChannelReader.scan(@fixtures_dir)
      assert result.total > 0
      {min_timestamp, max_timestamp} = result.timestamp_range
      assert min_timestamp <= max_timestamp
      assert length(result.messages) == result.total
    end

    test "messages are sorted ascending by timestamp and filename" do
      {:ok, result} = ChannelReader.scan(@fixtures_dir)
      keys = Enum.map(result.messages, fn {file, data} -> {data["timestamp"], file} end)
      assert keys == Enum.sort(keys)
    end

    test "supports mixed legacy seq and new ULID messages" do
      tmp = new_dir!()

      File.write!(
        Path.join(tmp, "00001-planner-architect.md"),
        message("id: 01KVD6GF0V4XBT269H8EBPWTKW\nseq: 1", "2026-06-13T00:00:00Z")
      )

      File.write!(
        Path.join(tmp, "01KV6BPDNNDCBMEWER3ZFKQHVG-coder-tester.md"),
        message("id: 01KV6BPDNNDCBMEWER3ZFKQHVG", "2026-06-14T00:00:00Z")
      )

      assert {:ok, result} = ChannelReader.scan(tmp)

      assert Enum.map(result.messages, &elem(&1, 0)) == [
               "00001-planner-architect.md",
               "01KV6BPDNNDCBMEWER3ZFKQHVG-coder-tester.md"
             ]

      assert result.timestamp_range == {"2026-06-13T00:00:00Z", "2026-06-14T00:00:00Z"}

      File.rm_rf!(tmp)
    end

    test "each message is a {filename, data} tuple" do
      {:ok, result} = ChannelReader.scan(@fixtures_dir)
      assert [{filename, data} | _] = result.messages
      assert is_binary(filename)
      assert is_map(data)
      assert Map.has_key?(data, "kind")
    end

    test "attaches markdown body for lifecycle scanners" do
      {:ok, result} = ChannelReader.scan(@fixtures_dir)

      assert {_filename, data} =
               Enum.find(result.messages, fn {_filename, data} -> data["seq"] == 50 end)

      assert data["_body"] =~ "Gate 1 approved"
    end

    test "skips non-message files (README, invalid front matter)" do
      # fixtures dir has no README but does have valid coordination messages
      # This test confirms invalid files are skipped rather than causing error
      {:ok, result} = ChannelReader.scan(@fixtures_dir)

      Enum.each(result.messages, fn {_file, data} ->
        assert Map.has_key?(data, "seq") or Map.has_key?(data, "id")
      end)
    end

    test "returns empty result for directory with no valid messages" do
      tmp = System.tmp_dir!() |> Path.join("agov_test_empty_#{:rand.uniform(99999)}")
      File.mkdir_p!(tmp)
      File.write!(Path.join(tmp, "README.md"), "# not a coordination message\n")

      assert {:ok, %{messages: [], total: 0, timestamp_range: nil}} = ChannelReader.scan(tmp)

      File.rm_rf!(tmp)
    end

    test "returns error for missing directory" do
      assert {:error, :dir_not_found} = ChannelReader.scan("/nonexistent/path/agov")
    end
  end

  defp new_dir! do
    tmp = System.tmp_dir!() |> Path.join("agov_test_channel_#{:rand.uniform(99999)}")
    File.rm_rf!(tmp)
    File.mkdir_p!(tmp)
    tmp
  end

  defp message(identifier, timestamp) do
    """
    ---
    kind: note
    from: coder-tester
    to: all
    #{identifier}
    timestamp: #{timestamp}
    immutable: true
    references-to: []
    topic: mixed-format
    requires-ack: false
    artifacts_owned: []
    ---

    Fixture.
    """
  end
end
