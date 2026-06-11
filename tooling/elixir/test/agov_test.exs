defmodule AgovTest do
  use ExUnit.Case
  doctest Agov

  test "greets the world" do
    assert Agov.hello() == :world
  end
end
