defmodule MatchingBracketsTest do
  use ExUnit.Case

  test "paired square brackets" do
    assert MatchingBrackets.check_brackets("[]")
  end

  test "empty string" do
    assert MatchingBrackets.check_brackets("")
  end

  test "unpaired brackets" do
    refute MatchingBrackets.check_brackets("[[")
  end

  test "wrong ordered brackets" do
    refute MatchingBrackets.check_brackets("}{")
  end

  test "wrong closing bracket" do
    refute MatchingBrackets.check_brackets("{]")
  end

  test "paired with whitespace" do
    assert MatchingBrackets.check_brackets("{ }")
  end

  test "partially paired brackets" do
    refute MatchingBrackets.check_brackets("{[])")
  end

  test "simple nested brackets" do
    assert MatchingBrackets.check_brackets("{[]}")
  end

  test "several paired brackets" do
    assert MatchingBrackets.check_brackets("{}[]")
  end

  test "paired and nested brackets" do
    assert MatchingBrackets.check_brackets("([{}({}[])])")
  end

  test "unopened closing brackets" do
    refute MatchingBrackets.check_brackets("{[)][]}")
  end

  test "unpaired and nested brackets" do
    refute MatchingBrackets.check_brackets("([{])")
  end

  test "paired and wrong nested brackets" do
    refute MatchingBrackets.check_brackets("[({]})")
  end

  test "paired and incomplete brackets" do
    refute MatchingBrackets.check_brackets("{}[")
  end

  test "too many closing brackets" do
    refute MatchingBrackets.check_brackets("[]]")
  end

  test "math expression" do
    assert MatchingBrackets.check_brackets("(((185 + 223.85) * 15) - 543)/2")
  end

  test "complex latex expression" do
    assert MatchingBrackets.check_brackets(
             "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \\end{array}\\right)"
           )
  end
end
