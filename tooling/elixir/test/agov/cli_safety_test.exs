defmodule Agov.CliSafetyTest do
  use ExUnit.Case, async: true

  alias Agov.CliSafety

  test "blocks root uid unless explicitly allowed" do
    assert CliSafety.blocked_root?(0, nil)
    assert CliSafety.blocked_root?(0, "")
    assert CliSafety.blocked_root?(0, "0")
    assert CliSafety.blocked_root?(0, "false")
  end

  test "allows root uid only for explicit allow values" do
    refute CliSafety.blocked_root?(0, "1")
    refute CliSafety.blocked_root?(0, "true")
    refute CliSafety.blocked_root?(0, "YES")
    refute CliSafety.blocked_root?(0, " on ")
  end

  test "does not block non-root or unknown uid" do
    refute CliSafety.blocked_root?(1000, nil)
    refute CliSafety.blocked_root?(nil, nil)
  end
end
