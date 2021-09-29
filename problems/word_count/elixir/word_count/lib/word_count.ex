defmodule WordCount do
  def count(filename, n) do
    filename
    |> count_words_in_file()
    |> get_top_n_words(n)
    |> display_word_freq_pairs()
  end

  def count_words_in_file(filename) do
    filename
    |> File.stream!()
    |> Enum.reduce(%{}, fn line, counter ->
      count_words_in_line(line, counter)
    end)
  end

  def get_top_n_words(counter, n) do
    counter
    |> Enum.sort_by(fn {_word, freq} -> freq end, :desc)
    |> Enum.take(n)
  end

  def display_word_freq_pairs(word_freq_pairs) do
    IO.write("Rank\tWord\tFrequency\n")

    word_freq_pairs
    |> Enum.with_index(1)
    |> Enum.each(fn {{word, freq}, position} ->
      IO.write("#{position}\t#{word}\t#{freq}\n")
    end)
  end

  defp count_words_in_line(line, counter) do
    line
    |> String.trim()
    |> String.split(" ")
    |> Enum.reduce(counter, fn word, counter ->
      word = process_word(word)
      count = counter[word] || 0
      if word != "", do: Map.put(counter, word, count + 1), else: counter
    end)
  end

  defp process_word(word) do
    word
    |> String.downcase()
    |> String.replace(~r/[^a-zà-ÿ]/, "")
  end
end
