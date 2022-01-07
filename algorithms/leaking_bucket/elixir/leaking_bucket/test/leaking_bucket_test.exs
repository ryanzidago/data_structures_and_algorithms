defmodule LeakingBucketTest do
  use ExUnit.Case
  doctest LeakingBucket

  test "greets the world" do
    assert LeakingBucket.hello() == :world
  end
end
