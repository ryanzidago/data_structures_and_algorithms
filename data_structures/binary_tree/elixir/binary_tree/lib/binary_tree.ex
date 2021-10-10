defmodule BinaryTree do
  defstruct [:val, :left, :right]

  def invert(nil), do: nil

  def invert(%__MODULE__{left: left, right: right} = node) do
    %{node | left: invert(right), right: invert(left)}
  end

  def symmetric?(%__MODULE__{left: nil, right: nil}), do: true
  def symmetric?(%__MODULE__{left: nil}), do: false
  def symmetric?(%__MODULE__{right: nil}), do: false
  def symmetric?(%__MODULE__{} = node), do: node.left == invert(node.right)
end
