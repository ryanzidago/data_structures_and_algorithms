defmodule Stack do
  @type t :: %__MODULE__{elements: list()}
  @enforce_keys [:elements]
  defstruct elements: []

  @spec new() :: Stack.t()
  def new() do
    %__MODULE__{elements: []}
  end

  @spec push(Stack.t(), any()) :: Stack.t()
  def push(%__MODULE__{elements: elements}, element) do
    %__MODULE__{elements: [element | elements]}
  end

  @spec pop(Stack.t()) :: {any(), Stack.t()}
  def pop(%__MODULE__{elements: []} = stack) do
    {nil, stack}
  end

  def pop(%__MODULE__{elements: [head | tail]}) do
    {head, %__MODULE__{elements: tail}}
  end
end

defmodule MatchingBrackets do
  @doc """
  Checks that all the brackets and braces in the string are matched correctly, and nested correctly
  """
  @brackets %{"(" => ")", "[" => "]", "{" => "}"}
  @opening_brackets Map.keys(@brackets)
  @closing_brackets Map.values(@brackets)
  @remove_non_bracket_chars ~r/[^\(\)\[\]\{\}]/

  @spec check_brackets(String.t()) :: boolean
  def check_brackets(str) do
    str = String.replace(str, @remove_non_bracket_chars, "")
    do_check_brackets(Stack.new(), str)
  end

  defp do_check_brackets(%Stack{elements: []}, "") do
    true
  end

  defp do_check_brackets(%Stack{}, "") do
    false
  end

  defp do_check_brackets(%Stack{} = stack, <<bracket::binary-size(1), rest::binary>> = _str)
       when bracket in @opening_brackets do
    stack
    |> Stack.push(bracket)
    |> do_check_brackets(rest)
  end

  defp do_check_brackets(%Stack{} = stack, <<bracket::binary-size(1), rest::binary>> = _str)
       when bracket in @closing_brackets do
    {maybe_matching_opening_bracket, stack} = Stack.pop(stack)

    if bracket == Map.get(@brackets, maybe_matching_opening_bracket) do
      do_check_brackets(stack, rest)
    else
      false
    end
  end

  defp do_check_brackets(%Stack{} = stack, <<_bracket::binary-size(1), rest::binary>> = _str) do
    do_check_brackets(stack, rest)
  end
end
