defmodule Zipper.ListZipperTest do
  use ExUnit.Case, async: true

  alias Zipper.ListZipper

  describe "new/0" do
    test "returns an empty list zipper" do
      assert {[], []} == ListZipper.new()
    end
  end

  describe "from_list/1" do
    test "creates a ListZipper from a list" do
      assert {[], [1, 2, 3]} == ListZipper.from_list([1, 2, 3])
      assert {[], ~w(hello world)} == ListZipper.from_list(~w(hello world))
      assert {[], [[[]]]} == ListZipper.from_list([[[]]])
    end
  end

  describe "from_range/1" do
    test "creates a ListZipper from a range" do
      assert {[], [1, 2, 3]} == ListZipper.from_range(1..3)
    end
  end

  describe "to_list/1" do
    test "creates a list from a ListZipper" do
      assert [1, 2, 3] == ListZipper.to_list({[], [1, 2, 3]})
      assert ~w(hello world) == ListZipper.to_list({[], ~w(hello world)})
      assert [[[]]] == ListZipper.to_list({[], [[[]]]})

      lzip = {[3, 2, 1], [4, 5, 6, 7, 8, 9, 10]}
      assert Enum.to_list(1..10) == ListZipper.to_list(lzip)
    end
  end

  describe "prev/1" do
    test "points to the element before the current element in the list zipper" do
      lzip = {[3, 2, 1], [4, 5, 6, 7, 8, 9, 10]}
      assert lzip = {[2, 1], [3, 4, 5, 6, 7, 8, 9, 10]} = ListZipper.prev(lzip)
      assert lzip = {[1], [2, 3, 4, 5, 6, 7, 8, 9, 10]} = ListZipper.prev(lzip)
      assert _lzip = {[], [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]} = ListZipper.prev(lzip)
    end

    test "returns the same zipper if there is no previous element" do
      lzip = {[], [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]}
      assert lzip == ListZipper.prev(lzip)
    end
  end

  describe "current/1" do
    test "points to the current element in the list zipper" do
      lzip = {[3, 2, 1], [4, 5, 6, 7, 8, 9, 10]}
      assert 4 = ListZipper.current(lzip)
    end

    test "returns `nil` if there is no current element" do
      lzip = {[3, 2, 1], []}
      assert is_nil(ListZipper.current(lzip))
    end
  end

  describe "next/1" do
    test "points to the element after the current element in the list zipper" do
      lzip = {[3, 2, 1], [4, 5, 6, 7, 8, 9, 10]}
      assert lzip = {[4, 3, 2, 1], [5, 6, 7, 8, 9, 10]} = ListZipper.next(lzip)
      assert lzip = {[5, 4, 3, 2, 1], [6, 7, 8, 9, 10]} = ListZipper.next(lzip)
      assert _lzip = {[6, 5, 4, 3, 2, 1], [7, 8, 9, 10]} = ListZipper.next(lzip)
    end

    test "returns the same list zipper if there is no next element" do
      lzip = {[5, 4, 3, 2, 1], []}
      assert lzip == ListZipper.next(lzip)
    end
  end

  describe "replace/2" do
    test "replaces the current element by a value" do
      lzip = {[3, 2, 1], [4, 5, 6, 7, 8, 9, 10]}
      assert {[3, 2, 1], ["hello", 5, 6, 7, 8, 9, 10]} == ListZipper.replace(lzip, "hello")
    end

    test "if the current element does not exist, value is put in a list" do
      lzip = {[3, 2, 1], []}
      assert {[3, 2, 1], ["hello"]} == ListZipper.replace(lzip, "hello")
    end
  end

  describe "put/2" do
    test "puts a value into the list zipper" do
      lzip = {[], []}
      assert lzip = {[], [1]} = ListZipper.put(lzip, 1)
      assert lzip = {[], [2, 1]} = ListZipper.put(lzip, 2)
      assert _lzip = {[], [3, 2, 1]} = ListZipper.put(lzip, 3)
    end
  end

  describe "delete/1" do
    test "deletes the current value from the zipper" do
      lzip = {[3, 2, 1], [4, 5, 6, 7, 8, 9, 10]}
      assert lzip = {[3, 2, 1], [5, 6, 7, 8, 9, 10]} = ListZipper.delete(lzip)
      assert lzip = {[3, 2, 1], [6, 7, 8, 9, 10]} = ListZipper.delete(lzip)
      assert _lzip = {[3, 2, 1], [7, 8, 9, 10]} = ListZipper.delete(lzip)
    end

    test "returns the same list zipper if there is no current value" do
      lzip = {[3, 2, 1], []}
      assert lzip == ListZipper.delete(lzip)
    end
  end
end
