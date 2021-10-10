defmodule TrieTest do
  use ExUnit.Case

  setup do
    trie = %Trie{
      children: %{
        "c" => %Trie{
          children: %{
            "a" => %Trie{
              children: %{
                "t" => %Trie{
                  children: %{
                    "__end__" => nil
                  }
                }
              }
            }
          }
        }
      }
    }

    trie_for_prefix = %Trie{
      children: %{
        "c" => %Trie{
          children: %{
            "a" => %Trie{
              children: %{
                "t" => %Trie{
                  children: %{
                    "__end__" => nil
                  }
                },
                "__end__" => nil
              }
            }
          }
        }
      }
    }

    {
      :ok,
      trie: trie, trie_for_prefix: trie_for_prefix
    }
  end

  describe "new/0" do
    test "creates a new trie" do
      assert %Trie{children: %{}} == Trie.new()
    end
  end

  describe "get/2" do
    test "get the trie at a given word/prefix", %{trie: _trie} do
    end

    test "returns `nil` if the given word/prefix does not exist within the trie", %{trie: trie} do
      assert is_nil(Trie.get(trie, "basketball"))
      assert is_nil(Trie.get(trie, "catastrophic"))
    end
  end

  describe "put/2" do
    test "puts into an empty trie", %{trie: expected} do
      trie = Trie.new()
      outcome = Trie.put(trie, "cat")
      assert outcome == expected
    end

    test "puts a word into a trie that already contains the word", %{trie: expected} do
      outcome = Trie.put(expected, "cat")
      assert outcome == expected
    end

    test "puts a word that is a prefix to another word into a trie", %{
      trie: trie,
      trie_for_prefix: expected
    } do
      outcome = Trie.put(trie, "ca")
      assert outcome == expected
    end
  end

  describe "has_word?/2" do
    test "returns `true` if the given word is within the trie", %{trie: trie} do
      assert true == Trie.has_word?(trie, "cat")
    end

    test "returns `false` if the given word is not within the trie", %{trie: trie} do
      assert false == Trie.has_word?(trie, "ca")
      assert false == Trie.has_word?(trie, "cab")
      assert false == Trie.has_word?(trie, "catastrophic")
    end
  end

  describe "has_prefix?/2" do
    test "returns `true` if the given prefix is within the trie", %{trie: trie} do
      assert true == Trie.has_prefix?(trie, "c")
      assert true == Trie.has_prefix?(trie, "ca")
    end

    test "returns `true` even if the prefix is the entire word in the trie", %{trie: trie} do
      assert true == Trie.has_prefix?(trie, "cat")
    end

    test "returns `false` if the given prefix is not within the trie", %{trie: trie} do
      assert false == Trie.has_prefix?(trie, "ball")
    end
  end
end