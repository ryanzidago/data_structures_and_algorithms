defmodule Frequency do
  @doc """
  Count letter frequency in parallel.

  Returns a map of characters to frequencies.

  The number of worker processes to use can be set with 'workers'.
  """
  @non_alphanum ~r/[^a-zà-ÿ]/

  @spec frequency([String.t()], pos_integer) :: map
  def frequency([], _), do: %{}

  def frequency(texts, workers) when is_list(texts) or is_struct(texts, File.Stream) do
    task_results =
      Task.async_stream(
        texts,
        fn text ->
          text
          |> process_string()
          |> Enum.frequencies()
        end,
        max_concurrency: workers
      )

    return_result(task_results)
  end

  defp process_string(text) do
    text
    |> String.downcase()
    |> String.replace(@non_alphanum, "", trim: true)
    |> String.graphemes()
  end

  defp return_result(task_results) do
    Enum.reduce(task_results, %{}, fn {:ok, sub_freq_map}, freq_map ->
      Map.merge(freq_map, sub_freq_map, fn _key, freq, sub_freq ->
        freq + sub_freq
      end)
    end)
  end
end
