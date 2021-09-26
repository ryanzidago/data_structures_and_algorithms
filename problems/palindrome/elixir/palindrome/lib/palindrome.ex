defmodule Palindrome do
  def palindrome?(string) do
    alphanumeric_string =
      string
      |> String.downcase()
      |> String.replace(~r/[^a-z0-9]/, "")

    alphanumeric_string == reverse(alphanumeric_string)
  end

  defp reverse(string) do
    string
    |> String.split("", trim: true)
    |> Enum.reverse()
    |> Enum.join()
  end
end
