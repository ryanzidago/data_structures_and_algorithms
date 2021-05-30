defmodule TreeNode do
  defstruct [:val, :left, :right]

  def new(val \\ nil) do
    %__MODULE__{val: val}
  end
end

defmodule BinarySearchTree do
  def insert(%TreeNode{val: val} = current_node, val) do
    current_node
  end

  def insert(%TreeNode{val: val, left: nil} = current_node, element) when element < val do
    %TreeNode{current_node | left: TreeNode.new(element)}
  end

  def insert(%TreeNode{val: val, left: left} = current_node, element) when element < val do
    %TreeNode{current_node | left: insert(left, element)}
  end

  def insert(%TreeNode{val: val, right: nil} = current_node, element) when element > val do
    %TreeNode{current_node | right: TreeNode.new(element)}
  end

  def insert(%TreeNode{val: val, right: right} = current_node, element) when element > val do
    %TreeNode{current_node | right: insert(right, element)}
  end

  def search(%TreeNode{val: val} = current_node, val) do
    current_node
  end

  def search(%TreeNode{val: val, left: left}, element)
      when element < val
      when not is_nil(left) do
    search(left, element)
  end

  def search(%TreeNode{val: val, right: right}, element)
      when element > val
      when not is_nil(right) do
    search(right, element)
  end

  def search(_, _) do
    nil
  end
end
