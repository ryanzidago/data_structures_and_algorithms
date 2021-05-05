defmodule FibTest do
  use ExUnit.Case
  doctest Fib

  test "greets the world" do
    assert Fib.hello() == :world
  end
end
