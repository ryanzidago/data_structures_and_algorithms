defmodule Zipper.ListZipper do
  defguard is_range(range) when is_struct(range, Range)

  def new, do: {[], []}

  def from_list(list) when is_list(list), do: {[], list}

  def from_range(range) when is_range(range), do: from_list(Enum.to_list(range))

  def to_list({prev, next}), do: Enum.reverse(prev) ++ next

  def prev({[], next}), do: {[], next}
  def prev({[head | tail], next}), do: {tail, [head | next]}

  def get({_, []}), do: nil
  def get({_, [current | _]}), do: current

  def pop({_, []} = lzip), do: {nil, lzip}
  def pop({prev, [current | tail]}), do: {current, {prev, tail}}

  def next({prev, []}), do: {prev, []}
  def next({prev, [head | tail]}), do: {[head | prev], tail}

  def replace({prev, []}, val), do: {prev, [val]}
  def replace({prev, [_ | next]}, val), do: {prev, [val | next]}

  def put({prev, next}, val), do: {prev, [val | next]}

  def delete({prev, []}), do: {prev, []}
  def delete({prev, [_ | next]}), do: {prev, next}
end
