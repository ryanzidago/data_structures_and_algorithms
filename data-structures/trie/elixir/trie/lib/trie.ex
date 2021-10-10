defmodule Trie do
  defstruct children: %{}

  def new, do: %Trie{}

  def get(trie, word) do
    graphemes = String.graphemes(word)
    do_get(trie, graphemes)
  end

  defp do_get(trie, []), do: trie

  defp do_get(trie, [char | chars]) do
    if Map.has_key?(trie.children, char) do
      do_get(trie.children[char], chars)
    end
  end

  def put(trie, word) do
    graphemes = String.graphemes(word)
    do_put(trie, graphemes)
  end

  defp do_put(%Trie{children: children} = trie, []) do
    %Trie{trie | children: Map.put_new(children, "__end__", nil)}
  end

  defp do_put(trie, [char | chars]) do
    if Map.has_key?(trie.children, char) do
      %Trie{children: %{char => do_put(trie.children[char], chars)}}
    else
      %Trie{children: %{char => do_put(Trie.new(), chars)}}
    end
  end

  def has_word?(trie, word) do
    graphemes = String.graphemes(word)
    do_has_word?(trie, graphemes)
  end

  defp do_has_word?(%Trie{children: children} = _trie, []), do: {"__end__", nil} in children

  defp do_has_word?(%Trie{} = trie, [char | chars]) do
    Map.has_key?(trie.children, char) and do_has_word?(trie.children[char], chars)
  end

  def has_prefix?(trie, word) do
    graphemes = String.graphemes(word)
    do_has_prefix?(trie, graphemes)
  end

  defp do_has_prefix?(%Trie{children: _children}, []), do: true

  defp do_has_prefix?(trie, [char | chars]) do
    Map.has_key?(trie.children, char) and do_has_prefix?(trie.children[char], chars)
  end
end
