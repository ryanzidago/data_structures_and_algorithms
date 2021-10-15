defmodule Zipper.TernaryTreeZipperTest do
  use ExUnit.Case, async: true

  alias Zipper.TernaryTreeZipper

  setup do
    ternary_tree =
      TernaryTree.new(
        val: 100,
        left: TernaryTree.new(val: 25, left: TernaryTree.new(val: 10)),
        mid: TernaryTree.new(val: 50, right: TernaryTree.new(val: 75)),
        right: TernaryTree.new(val: 200)
      )

    ttreezip = TernaryTreeZipper.new(ternary_tree)

    {
      :ok,
      ternary_tree: ternary_tree, ttreezip: ttreezip
    }
  end

  describe "new/1" do
    test "creates a new zipper with an empty thread and a ternary tree inside" do
      assert {[], ternary_tree} =
               TernaryTreeZipper.new(TernaryTree.new(val: 100, left: nil, mid: nil, right: nil))

      assert ternary_tree == %TernaryTree{val: 100, left: nil, mid: nil, right: nil}
    end
  end

  describe "leaf?/1" do
    test "returns `true` if the current node of the zipper is a leaf node" do
      ttreezip = {[], TernaryTree.new(val: 1)}
      assert TernaryTreeZipper.leaf?(ttreezip)
    end

    test "returns `false` if the current node of the zipper is not a leaf node" do
      ttreezip = {[], TernaryTree.new(val: 1, left: TernaryTree.new())}
      assert false == TernaryTreeZipper.leaf?(ttreezip)
    end
  end

  describe "get/1" do
    test "gets the value of the zipper's current node" do
      ttreezip = {[], TernaryTree.new(val: 1)}
      assert 1 == TernaryTreeZipper.get(ttreezip)
    end
  end

  describe "put/1" do
    test "replace the zipper's current node with a new ternary tree node, or put it into the zipper" do
      ttreezip = {[], TernaryTree.new(val: 1)}
      ttreezip = TernaryTreeZipper.put(ttreezip, TernaryTree.new(val: 2))
      assert {[], %TernaryTree{val: 2}} == ttreezip
    end
  end

  describe "left/1" do
    test "visits the left node of the ternary tree and puts the non visited nodes into the zipper's thread",
         %{ternary_tree: init_ttree, ttreezip: ttreezip} do
      assert {thread, ttree} = TernaryTreeZipper.left(ttreezip)

      expected_thread = %TernaryTree{init_ttree | left: nil}
      assert thread == [left: expected_thread]

      expected_ttree = init_ttree.left
      assert ttree == expected_ttree
    end

    test "returns `nil` if the zipper's current node has no left child" do
      ttree = TernaryTree.new(val: 100, mid: TernaryTree.new(), right: TernaryTree.new())
      ttreezip = TernaryTreeZipper.new(ttree)
      assert nil == TernaryTreeZipper.left(ttreezip)
    end
  end

  describe "mid/1" do
    test "visits the zipper's current node's mid node and append the non visited node into the thread",
         %{ternary_tree: init_ttree, ttreezip: ttreezip} do
      assert {thread, ttree} = TernaryTreeZipper.mid(ttreezip)

      expected_thread = %TernaryTree{init_ttree | mid: nil}
      assert thread == [mid: expected_thread]

      expected_ttree = init_ttree.mid
      assert ttree == expected_ttree
    end

    test "returns `nil``if the zipper's current node has no mid child" do
      ttree = TernaryTree.new(val: 100, left: TernaryTree.new(), right: TernaryTree.new())
      ttreezip = TernaryTreeZipper.new(ttree)
      assert nil == TernaryTreeZipper.mid(ttreezip)
    end
  end

  describe "right/1" do
    test "visits the zipper's current node's right node and append the non visited node into the thread",
         %{ternary_tree: init_ttree, ttreezip: ttreezip} do
      assert {thread, ttree} = TernaryTreeZipper.right(ttreezip)

      expected_thread = %TernaryTree{init_ttree | right: nil}
      assert thread == [right: expected_thread]

      expected_ttree = init_ttree.right
      assert ttree == expected_ttree
    end

    test "returns `nil` if the zipper's current node has no right child" do
      ttree = TernaryTree.new(val: 100, left: TernaryTree.new(), mid: TernaryTree.new())
      ttreezip = TernaryTreeZipper.new(ttree)
      assert nil == TernaryTreeZipper.right(ttreezip)
    end
  end

  describe "top/1" do
    test "follows the thread", %{ternary_tree: ternary_tree, ttreezip: ttreezip} do
      assert ttreezip ==
               ternary_tree
               |> TernaryTreeZipper.new()
               |> TernaryTreeZipper.mid()
               |> TernaryTreeZipper.right()
               |> TernaryTreeZipper.top()
               |> TernaryTreeZipper.top()
    end

    test "returns `nil` if the thread is empty" do
      assert nil == TernaryTreeZipper.top({[], TernaryTree.new()})
    end
  end

  describe "set_left_branch/2" do
    test "visit the left child and replace it with a new ternary tree, follows the thread up", %{
      ttreezip: ttreezip
    } do
      left = TernaryTree.new(val: 1_000)

      assert {[], updated_ternary_tree} = TernaryTreeZipper.set_left_branch(ttreezip, left)
      assert updated_ternary_tree.left == left
    end
  end

  describe "set_mid_branch/2" do
    test "visit the mid child and replace it with a new ternary tree, follows the thread up", %{
      ttreezip: ttreezip
    } do
      mid = TernaryTree.new(val: 1_000)

      assert {[], updated_ternary_tree} = TernaryTreeZipper.set_mid_branch(ttreezip, mid)
      assert updated_ternary_tree.mid == mid
    end
  end

  describe "set_right_branch/2" do
    test "visit the right child and replace it with a new ternary tree, follows the thread up", %{
      ttreezip: ttreezip
    } do
      right = TernaryTree.new(val: 1_000)

      assert {[], updated_ternary_tree} = TernaryTreeZipper.set_right_branch(ttreezip, right)
      assert updated_ternary_tree.right == right
    end
  end
end
