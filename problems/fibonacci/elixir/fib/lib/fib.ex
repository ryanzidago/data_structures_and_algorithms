defmodule FibUnfold do
  def first_nth(n), do: Enum.take(fib(), n)

  def nth(n), do: Enum.at(fib(), n)

  def fib do
    Stream.unfold({0, 1}, fn {current, next} -> {current, {next, current + next}) end)
  end
end

defmodule FibRec do
  def nth(n), when n < 2, do: n
  def nth(n), do: nth(n - 2) + nth(n - 1)
end
