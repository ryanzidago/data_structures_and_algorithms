defmodule Kadane do
  def sum_of_maximum_sublist([]), do: 0

  def sum_of_maximum_sublist(elements) do
    elements
    |> do_sum_of_maximum_sublist()
    |> elem(1)
  end

  defp do_sum_of_maximum_sublist([head | tail] = _elements) do
    Enum.reduce(tail, {head, head}, fn head, {current_sum, max_sum} when is_number(head) ->
      current_sum = max(head, current_sum + head)
      max_sum = max(max_sum, current_sum)

      {current_sum, max_sum}
    end)
  end
end
