defmodule KadaneTest do
  use ExUnit.Case

  describe "sum_of_maximum_sublist/1" do
    test "returns the sum of the maximum contiguous sublist" do
      assert 6 == Kadane.sum_of_maximum_sublist([-2, 1, -3, 4, -1, 2, 1, -5, 4])
      assert 1 == Kadane.sum_of_maximum_sublist([1])
      assert 23 == Kadane.sum_of_maximum_sublist([5, 4, -1, 7, 8])
      assert 0 == Kadane.sum_of_maximum_sublist([])
      assert 3 == Kadane.sum_of_maximum_sublist([1, 2, -5, 2])
    end
  end
end
