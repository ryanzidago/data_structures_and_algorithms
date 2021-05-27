defmodule StableMarriage do
  def match(men, women, state \\ %{}, is_stable \\ false)

  def match(men, _women, state, true) do
    state
    |> Map.to_list()
    |> Enum.slice(0..(length(Map.keys(men)) - 1))
  end

  def match(men, women, state, false) do
    {new_men, state} =
      Enum.reduce(men, {men, state}, fn {m1, [f | preferences]}, {men, state} ->
        is_m1_single = !state[m1]
        is_f_single = !state[f]
        m2 = if is_f_single, do: nil, else: state[f]

        new_state =
          case {is_m1_single, is_f_single, is_m1_prefered(women, f, m1, m2)} do
            {true, true, true} -> state |> Map.put(m1, f) |> Map.put(f, m1)
            {true, false, true} -> state |> Map.delete(m2) |> Map.put(m1, f) |> Map.put(f, m1)
            _ -> state
          end

        {_men = %{men | m1 => preferences}, new_state}
      end)

    match(new_men, women, state, is_stable(men, state))
  end

  def is_m1_prefered(_women, _woman, _m1, nil) do
    true
  end

  def is_m1_prefered(women, woman, m1, m2) do
    Enum.find_index(women[woman], &(&1 == m1)) <
      Enum.find_index(women[woman], &(&1 == m2))
  end

  def is_stable(men, state) do
    length(Map.keys(men)) * 2 == length(Map.keys(state))
  end
end
