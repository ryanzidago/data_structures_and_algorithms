defmodule BoyerMooreMajorityVote do
  @moduledoc """
  Source: https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm

  Time complexity: O(N)
  Space complexity: O(1)
  """
  def find_majority_element(list) do
    do_find_majority_element(list, 0, nil)
  end

  defp do_find_majority_element([], _count, candidate), do: candidate

  defp do_find_majority_element([head | tail], count = 0, _candidate) do
    do_find_majority_element(tail, count, head)
  end

  defp do_find_majority_element([candidate | tail], count, candidate) do
    do_find_majority_element(tail, count + 1, candidate)
  end

  defp do_find_majority_element([_head | tail], count, candidate) do
    do_find_majority_element(tail, count - 1, candidate)
  end
end
