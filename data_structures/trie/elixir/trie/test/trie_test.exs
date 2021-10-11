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

    another_trie = %Trie{
      children: %{
        "d" => %Trie{
          children: %{
            "o" => %Trie{
              children: %{
                "g" => %Trie{
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

    trie_with_two_words = %Trie{trie | children: Map.merge(trie.children, another_trie.children)}

    {
      :ok,
      trie: trie, trie_for_prefix: trie_for_prefix, trie_with_two_words: trie_with_two_words
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

    test "puts a word that is not a prefix to another word within a trie", %{
      trie_with_two_words: expected
    } do
      outcome = Trie.new() |> Trie.put("cat") |> Trie.put("dog")
      assert outcome == expected
    end

    test "puts a list of words into a trie" do
      outcome = Trie.put(Trie.new(), ~w(cat dog))
      assert outcome == Trie.new() |> Trie.put("cat") |> Trie.put("dog")
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

  describe "trie" do
    @tag :exclude_this_test
    test "works for a large file" do
      les_miserables_as_a_trie =
        "135-0.txt"
        |> File.stream!()
        |> Stream.map(fn line ->
          line
          |> String.trim()
          |> String.split(" ", trim: true)
          |> Enum.map(&process_word/1)
        end)
        |> Enum.reduce(Trie.new(), fn words, trie ->
          Trie.put(trie, words)
        end)

      assert Trie.has_word?(les_miserables_as_a_trie, "bishop")
      assert Trie.has_word?(les_miserables_as_a_trie, "monsieur")
      assert Trie.has_word?(les_miserables_as_a_trie, "revolution")

      refute Trie.has_word?(les_miserables_as_a_trie, "aufladekabel")
      refute Trie.has_word?(les_miserables_as_a_trie, "borboleta")
      refute Trie.has_word?(les_miserables_as_a_trie, "ordinateur")
    end
  end

  defp process_word(word) do
    word
    |> String.downcase()
    |> String.replace(~r/[^a-zà-ÿ]/, "")
  end
end
