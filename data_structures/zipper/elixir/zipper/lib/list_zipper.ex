defmodule Zipper.ListZipper do
  def new, do: {[], []}

  def from_list(list) when is_list(list), do: {[], list}

  def from_range(%Range{} = range), do: from_list(Enum.to_list(range))

  def to_list({prev, next}), do: Enum.reverse(prev) ++ next

  def prev({[], next}), do: {[], next}
  def prev({[head | tail], next}), do: {tail, [head | next]}

  def current({_, []}), do: nil
  def current({_, [current | _]}), do: current

  def next({prev, []}), do: {prev, []}
  def next({prev, [head | tail]}), do: {[head | prev], tail}

  def replace({prev, []}, val), do: {prev, [val]}
  def replace({prev, [_ | next]}, val), do: {prev, [val | next]}

  def insert({prev, next}, val), do: {prev, [val | next]}

  def delete({prev, []}), do: {prev, []}
  def delete({prev, [_ | next]}), do: {prev, next}
end
