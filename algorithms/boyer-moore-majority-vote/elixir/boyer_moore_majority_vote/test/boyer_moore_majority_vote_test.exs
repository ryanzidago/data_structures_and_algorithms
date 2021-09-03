defmodule BoyerMooreMajorityVoteTest do
  use ExUnit.Case

  import BoyerMooreMajorityVote

  describe "find_majority_element/1" do
    test "finds the majority element inside a list" do
      assert 3 == find_majority_element([3, 2, 3])
      assert 2 == find_majority_element([2, 2, 1, 1, 1, 2, 2])
      assert 7 == find_majority_element([7, 7, 5, 7, 5, 1, 5, 7, 5, 5, 7, 7, 7, 7, 7, 7])
      assert 5 == find_majority_element([7, 7, 5, 7, 5, 1, 5, 7, 5, 5, 7, 7, 5, 5, 5, 5])
      assert nil == find_majority_element([])
    end
  end
end
