defmodule Zipper.TernaryTreeZipper do
  def new(%TernaryTree{} = ternary_tree) do
    {[], ternary_tree}
  end

  def leaf?({_thread, %TernaryTree{left: nil, mid: nil, right: nil}}), do: true
  def leaf?({_thread, %TernaryTree{}}), do: false

  def get({_thread, %TernaryTree{val: val}}), do: val

  def put({thread, %TernaryTree{} = _replaced_ternary_tree}, %TernaryTree{} = ternary_tree) do
    {thread, ternary_tree}
  end

  def left({_thread, %TernaryTree{left: nil}}), do: nil

  def left({thread, %TernaryTree{val: val, left: left, mid: mid, right: right}}) do
    {[{:left, TernaryTree.new(val: val, mid: mid, right: right)} | thread], left}
  end

  def mid({_thread, %TernaryTree{mid: nil}}), do: nil

  def mid({thread, %TernaryTree{val: val, left: left, mid: mid, right: right}}) do
    {[{:mid, TernaryTree.new(val: val, left: left, right: right)} | thread], mid}
  end

  def right({_thread, %TernaryTree{right: nil}}), do: nil

  def right({thread, %TernaryTree{val: val, left: left, mid: mid, right: right}}) do
    {[{:right, TernaryTree.new(val: val, left: left, mid: mid)} | thread], right}
  end

  def top({[], %TernaryTree{} = _ternary_tree}), do: nil

  def top({[{:left, %TernaryTree{val: val, left: nil, mid: mid, right: right}} | thread], left}) do
    {thread, TernaryTree.new(val: val, left: left, mid: mid, right: right)}
  end

  def top({[{:mid, %TernaryTree{val: val, left: left, mid: nil, right: right}} | thread], mid}) do
    {thread, TernaryTree.new(val: val, left: left, mid: mid, right: right)}
  end

  def top({[{:right, %TernaryTree{val: val, left: left, mid: mid, right: nil}} | thread], right}) do
    {thread, TernaryTree.new(val: val, left: left, mid: mid, right: right)}
  end

  def set_left_branch(ternary_tree_zipper, ternary_tree) do
    ternary_tree_zipper
    |> left()
    |> put(ternary_tree)
    |> top()
  end

  def set_mid_branch(ternary_tree_zipper, ternary_tree) do
    ternary_tree_zipper
    |> mid()
    |> put(ternary_tree)
    |> top()
  end

  def set_right_branch(ternary_tree_zipper, ternary_tree) do
    ternary_tree_zipper
    |> right()
    |> put(ternary_tree)
    |> top()
  end
end
