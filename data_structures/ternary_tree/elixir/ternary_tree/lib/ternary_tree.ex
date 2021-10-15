defmodule TernaryTree do
  defstruct [:val, :left, :mid, :right]

  def new(params \\ []) do
    val = Keyword.get(params, :val)
    left = Keyword.get(params, :left)
    mid = Keyword.get(params, :mid)
    right = Keyword.get(params, :right)

    %TernaryTree{val: val, left: left, mid: mid, right: right}
  end
end
