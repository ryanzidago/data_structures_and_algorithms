defmodule Zipper.BinaryTreeZipperTest do
  use ExUnit.Case, async: true

  alias Zipper.BinaryTreeZipper

  setup do
    binary_tree = %BinaryTree{
      val: 1,
      left: %BinaryTree{
        val: 2,
        left: %BinaryTree{
          val: 3
        },
        right: %BinaryTree{
          val: 4
        }
      },
      right: %BinaryTree{
        val: 2,
        left: %BinaryTree{
          val: 4
        },
        right: %BinaryTree{
          val: 3
        }
      }
    }

    binary_tree_zipper = BinaryTreeZipper.new(binary_tree)

    {
      :ok,
      binary_tree: binary_tree, binary_tree_zipper: binary_tree_zipper
    }
  end

  describe "new/1" do
    test "creates a binary tree zipper pointing to the root of the tree", %{
      binary_tree: binary_tree
    } do
      assert {[], binary_tree} == BinaryTreeZipper.new(binary_tree)
    end
  end

  describe "leaf?/1" do
    test "returns `true` if the zipper points to a leaf node" do
      assert true ==
               BinaryTree.new(1, nil, nil)
               |> BinaryTreeZipper.new()
               |> BinaryTreeZipper.leaf?()
    end

    test "returns `false` if the zipper points to a node that isn't a leaf node" do
      assert false ==
               BinaryTree.new(1, BinaryTree.new(2), BinaryTree.new(3))
               |> BinaryTreeZipper.new()
               |> BinaryTreeZipper.leaf?()
    end
  end

  describe "current/1" do
    test "returns the value of the current node pointed by the zipper", %{
      binary_tree_zipper: binary_tree_zipper
    } do
      assert 1 = BinaryTreeZipper.current(binary_tree_zipper)
    end

    test "returns `nil` if there is no node pointed by the zipper" do
      current =
        %BinaryTree{}
        |> BinaryTreeZipper.new()
        |> BinaryTreeZipper.current()

      assert is_nil(current)
    end
  end

  describe "replace/2" do
    test "replaces the value of the current node pointed by the zipper ", %{
      binary_tree_zipper: binary_tree_zipper,
      binary_tree: binary_tree
    } do
      assert {[], %BinaryTree{binary_tree | val: 100}} ==
               BinaryTreeZipper.replace(binary_tree_zipper, 100)
    end
  end
end
