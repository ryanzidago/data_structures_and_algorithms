defmodule StableMarriage do
  @type man() :: String.t()
  @type woman() :: String.t()
  @type men() :: %{man() => [woman()]}
  @type women() :: %{woman() => [man()]}
  @type couples() :: [{man(), woman()}]

  @doc """
  Stable match men and women using the Gale-Shapley algorithm.
  """
  @spec match(men(), women()) :: couples()
  def match(men, women) do
    score = calculate_score(women)
    do_match(men, women, _state = %{}, score, _stable? = false)
  end

  # `state` is a map holding all the couples
  # if "a" and "z" are a couple
  # then `state` will be equal to %{"a" => "z", "z" => "a"}
  #
  # `score` is used to quickly find out which man prefers the woman
  #
  # `stable?` stops the algorithm as soon as every man is paired to a woman
  # it is done by checking that the number of keys in `state`
  # is equal to two times the number of keys in `men` (one time for the men themselves and another time for the women)
  defp do_match(men, _women, state, _score, true) do
    men_keys = Map.keys(men)
    for {k, v} <- state, k in men_keys, do: {k, v}
  end

  defp do_match(men, women, state, score, false) do
    {men, state} =
      Enum.reduce(men, {men, state}, fn
        {m1, [w | preferences]}, {men, state} ->
          if m1_single? = !state[m1] do
            w_single? = !state[w]
            m2 = if w_single?, do: nil, else: state[w]
            m1_prefered_to_m2? = if m2, do: score[w][m1] < score[w][m2], else: true

            new_state =
              case {m1_single?, w_single?, m1_prefered_to_m2?} do
                {true, true, true} -> state |> Map.put(m1, w) |> Map.put(w, m1)
                {true, false, true} -> state |> Map.delete(m2) |> Map.put(m1, w) |> Map.put(w, m1)
                _ -> state
              end

            {_men = %{men | m1 => preferences}, new_state}
          else
            {men, state}
          end
      end)

    do_match(men, women, state, score, stable?(men, state))
  end

  defp calculate_score(women) do
    for {w, prefs} <- women, into: %{} do
      prefs_with_score =
        prefs
        |> Enum.with_index()
        |> Enum.into(%{})

      {w, prefs_with_score}
    end
  end

  defp stable?(men, state) do
    length(Map.keys(men)) * 2 == length(Map.keys(state))
  end
end
