defmodule Zipper.BinaryTreeZipper do
  def new(%BinaryTree{} = binary_tree) do
    {[], binary_tree}
  end

  def leaf?({_thread, %BinaryTree{left: nil, right: nil}}), do: true
  def leaf?({_thread, %BinaryTree{}}), do: false

  def current({_thread, %BinaryTree{val: val}}), do: val
  def current({_thread, nil}), do: nil

  def replace({thread, %BinaryTree{} = binary_tree}, val) do
    {thread, %BinaryTree{binary_tree | val: val}}
  end

  def right({thread, %BinaryTree{val: val, left: left, right: right}}) do
    {[{:right, BinaryTree.new(val, left)} | thread], right}
  end

  def right({_thread, nil}), do: nil

  def left({thread, %BinaryTree{val: val, left: left, right: right}}) do
    {[{:left, BinaryTree.new(val, nil, right)} | thread], left}
  end

  def top({[{:left, %BinaryTree{val: val, left: nil, right: right}} | thread], left}) do
    {thread, BinaryTree.new(val, left, right)}
  end

  def top({[{:right, %BinaryTree{val: val, left: left, right: nil}} | thread], right}) do
    {thread, BinaryTree.new(val, left, right)}
  end

  def set_left_branch(binary_tree_zipper, val) do
    top(replace(left(binary_tree_zipper), val))
  end

  def set_right_branch(binary_tree_zipper, val) do
    top(replace(right(binary_tree_zipper), val))
  end
end
