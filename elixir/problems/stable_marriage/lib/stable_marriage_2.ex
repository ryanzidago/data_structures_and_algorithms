defmodule StableMarriage2 do
  def match(proposers, acceptors) do
    {proposers, acceptors} = {prepare(proposers), prepare(acceptors)}
    {proposers, _acceptors} = do_match(proposers, acceptors)
    present(proposers)
  end

  defp present(proposers) do
    for {p, %{partner: a}} <- proposers, do: {p, a}
  end

  defp prepare(prefmap) do
    Enum.into(prefmap, %{}, fn {name, prefs} ->
      {name, %{prefs: prefs, partner: nil}}
    end)
  end

  def do_match(proposers, acceptors, single_proposers \\ []) do
    single_proposers =
      Enum.reduce(proposers, single_proposers, fn
        {p_name, %{partner: nil}}, single_proposers ->
          [p_name | single_proposers]

        _, single_proposers ->
          single_proposers
      end)

    case single_proposers do
      [] ->
        {proposers, acceptors}

      single_proposers ->
        {proposers, acceptors} = round(proposers, acceptors, single_proposers)
        do_match(proposers, acceptors)
    end
  end

  defp round(proposers, acceptors, single_proposers) do
    Enum.reduce(single_proposers, {proposers, acceptors}, fn p_name, {proposers, acceptors} ->
      p = proposers[p_name]
      [a_name | less_prefered_acceptors] = p.prefs
      a = acceptors[a_name]

      case propose(a, p_name) do
        {:accept, nil} ->
          engage({proposers, acceptors}, {{p, p_name}, {a, a_name}})

        {:accept, jilted} ->
          engage({proposers, acceptors}, {{p, p_name}, {a, a_name}}, jilted)

        :reject ->
          reject({proposers, acceptors}, {{p, p_name, less_prefered_acceptors}, {a, a_name}})
      end
    end)
  end

  def engage({proposers, acceptors}, {{p, p_name}, {a, a_name}}) do
    proposers = Map.put(proposers, p_name, %{p | partner: a_name})
    acceptors = Map.put(acceptors, a_name, %{a | partner: p_name})
    {proposers, acceptors}
  end

  def engage({proposers, acceptors}, {{p, p_name}, {a, a_name}}, jilted) do
    proposers =
      Map.update!(proposers, jilted, fn %{prefs: prefs} ->
        %{partner: nil, prefs: prefs -- [a_name]}
      end)

    engage({proposers, acceptors}, {{p, p_name}, {a, a_name}})
  end

  def reject({proposers, acceptors}, {{p, p_name, less_prefered_acceptors}, {a, a_name}}) do
    proposers = Map.put(proposers, p_name, %{p | prefs: less_prefered_acceptors})
    acceptors = Map.put(acceptors, a_name, %{a | prefs: a.prefs -- [p_name]})
    {proposers, acceptors}
  end

  defp propose(%{partner: nil}, _p_name), do: {:accept, nil}
  defp propose(%{partner: partner, prefs: prefs}, p_name), do: propose(partner, prefs, p_name)
  defp propose(partner, [p_name | _], p_name), do: {:accept, partner}
  defp propose(partner, [partner | _], _p_name), do: :reject
  defp propose(partner, [_ | rest], p_name), do: propose(partner, rest, p_name)
end
