defmodule Stack do
  defstruct data: []

  def new(data \\ []) when is_list(data), do: %Stack{data: data}

  def push(%Stack{data: data} = stack, element) do
    %Stack{stack | data: [element | data]}
  end

  def pop(%Stack{data: []} = stack) do
    {nil, stack}
  end

  def pop(%Stack{data: [element | data]} = stack) do
    {element, %Stack{stack | data: data}}
  end
end
