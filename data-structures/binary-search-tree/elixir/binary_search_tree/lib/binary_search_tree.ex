defmodule BinarySearchTree do
  defstruct [:val, :left, :right]
  defguard is_leaf(node) when is_nil(node.left) and is_nil(node.right)

  def new(val) when not is_nil(val) do
    %__MODULE__{val: val}
  end

  def insert(%__MODULE__{val: val} = current_node, val) do
    current_node
  end

  def insert(%__MODULE__{val: val, left: nil} = current_node, element) when element < val do
    %__MODULE__{current_node | left: new(element)}
  end

  def insert(%__MODULE__{val: val, left: left} = current_node, element) when element < val do
    %__MODULE__{current_node | left: insert(left, element)}
  end

  def insert(%__MODULE__{val: val, right: nil} = current_node, element) when element > val do
    %__MODULE__{current_node | right: new(element)}
  end

  def insert(%__MODULE__{val: val, right: right} = current_node, element) when element > val do
    %__MODULE__{current_node | right: insert(right, element)}
  end

  def search(%__MODULE__{val: val} = current_node, val) do
    current_node
  end

  def search(%__MODULE__{val: val, left: left}, element)
      when element < val
      when not is_nil(left) do
    search(left, element)
  end

  def search(%__MODULE__{val: val, right: right}, element)
      when element > val
      when not is_nil(right) do
    search(right, element)
  end

  def search(_, _) do
    nil
  end

  def leaf?(node) when is_leaf(node), do: true
  def leaf?(_), do: false

  def pre_order_traversal(node, func \\ fn node -> node end)
  def pre_order_traversal(nil, _func), do: []

  def pre_order_traversal(%__MODULE__{val: val, left: left, right: right}, func)
      when is_function(func) do
    [func.(val)] ++ pre_order_traversal(left, func) ++ pre_order_traversal(right, func)
  end

  def in_order_traversal(node, func \\ fn node -> node end)
  def in_order_traversal(nil, _func), do: []

  def in_order_traversal(%__MODULE__{val: val, left: left, right: right}, func)
      when is_function(func) do
    in_order_traversal(left, func) ++ [func.(val)] ++ in_order_traversal(right, func)
  end

  def post_order_traversal(node, func \\ fn node -> node end)
  def post_order_traversal(nil, _func), do: []

  def post_order_traversal(%__MODULE__{val: val, left: left, right: right}, func)
      when is_function(func) do
    post_order_traversal(left, func) ++ post_order_traversal(right, func) ++ [func.(val)]
  end

  def min(%__MODULE__{val: val} = node) when is_leaf(node), do: val
  def min(%__MODULE__{left: left}), do: min(left)

  def max(%__MODULE__{val: val} = node) when is_leaf(node), do: val
  def max(%__MODULE__{right: right}), do: max(right)

  def invert(nil), do: nil

  def invert(%__MODULE__{left: left, right: right} = node) do
    %{node | left: invert(right), right: invert(left)}
  end

  def same_tree?(%__MODULE__{} = tree, %__MODULE__{} = tree), do: true
  def same_tree?(%__MODULE__{}, %__MODULE__{}), do: false
end
