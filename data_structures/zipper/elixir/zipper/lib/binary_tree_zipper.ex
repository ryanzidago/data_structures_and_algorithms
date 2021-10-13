defmodule Zipper.BinaryTreeZipper do
  def new(%BinaryTree{} = binary_tree) do
    {[], binary_tree}
  end

  def leaf?({_thread, %BinaryTree{left: nil, right: nil}}), do: true
  def leaf?({_thread, %BinaryTree{}}), do: false

  def get({_thread, %BinaryTree{val: val}}), do: val
  def get({_thread, nil}), do: nil

  def put({thread, %BinaryTree{} = _replaced_binary_tree}, binary_tree) do
    {thread, binary_tree}
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
    binary_tree_zipper
    |> left()
    |> put(val)
    |> top()

    # top(put(left(binary_tree_zipper), val))
  end

  def set_right_branch(binary_tree_zipper, val) do
    binary_tree_zipper
    |> right()
    |> put(val)
    |> top()

    # top(put(right(binary_tree_zipper), val))
  end
end
