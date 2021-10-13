defmodule Queue do
  defguard is_range(range) when is_struct(range, Range)
  defstruct data: {[], []}

  def new, do: %Queue{}
  def new(input) when is_list(input), do: %Queue{data: {input, []}}
  def new(input) when is_range(input), do: %Queue{data: {Enum.to_list(input), []}}

  def push(%Queue{data: {[], output}}, val), do: %Queue{data: {[val], output}}
  def push(%Queue{data: {input, output}}, val), do: %Queue{data: {[val | input], output}}

  def pop(%Queue{data: {[], []}} = queue), do: {nil, queue}
  def pop(%Queue{data: {input, []}}), do: pop(%Queue{data: {[], Enum.reverse(input)}})
  def pop(%Queue{data: {input, [head | output]}}), do: {head, %Queue{data: {input, output}}}

  def empty?(%Queue{data: {[], []}}), do: true
  def empty?(%Queue{data: {_, _}}), do: false
end
