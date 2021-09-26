defmodule PalindromeTest do
  use ExUnit.Case

  describe "palindrome?" do
    test "returns `true` is the string is a palindrome" do
      string = "A man, a plan, a canal: Panama"
      assert Palindrome.palindrome?(string)
    end

    test "returns `false` if the string is not a palindrome" do
      string = "race a car"
      refute Palindrome.palindrome?(string)
    end
  end
end
