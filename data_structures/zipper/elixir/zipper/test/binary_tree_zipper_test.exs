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

  describe "get/1" do
    test "gets the value of the current node pointed by the zipper", %{
      binary_tree_zipper: binary_tree_zipper
    } do
      assert 1 = BinaryTreeZipper.get(binary_tree_zipper)
    end

    test "returns `nil` if there is no node pointed by the zipper" do
      current =
        %BinaryTree{}
        |> BinaryTreeZipper.new()
        |> BinaryTreeZipper.get()

      assert is_nil(current)
    end
  end

  describe "put/2" do
    test "replaces the current node by a new node or put the node into the zipper if no current node existed ",
         %{
           binary_tree_zipper: binary_tree_zipper
         } do
      assert {[], BinaryTree.new(1, nil, nil)} ==
               BinaryTreeZipper.put(binary_tree_zipper, BinaryTree.new(1, nil, nil))
    end
  end

  describe "left/1" do
    test "appends a new node with the value and left child of the current node to the thread, returns the zipper with the updated thread and the left child",
         %{
           binary_tree_zipper: binary_tree_zipper,
           binary_tree: binary_tree
         } do
      assert binary_tree_zipper = BinaryTreeZipper.left(binary_tree_zipper)
      assert {thread, right} = binary_tree_zipper
      assert thread[:left] == BinaryTree.new(binary_tree.val, nil, binary_tree.right)
      assert right == binary_tree.left
    end

    test "returns `nil` if the current node has no left child" do
      assert is_nil(BinaryTreeZipper.left({[], BinaryTree.new(1, nil, BinaryTree.new(0))}))
    end
  end

  describe "right/1" do
    test "appends a new node with the value and left child of the current node to the thread, returns the zipper with the updated thread and the left child",
         %{
           binary_tree_zipper: binary_tree_zipper,
           binary_tree: binary_tree
         } do
      assert binary_tree_zipper = BinaryTreeZipper.right(binary_tree_zipper)
      assert {thread, left} = binary_tree_zipper
      assert thread[:right] == BinaryTree.new(binary_tree.val, binary_tree.left)
      assert left == binary_tree.right
    end

    test "returns `nil` if the current node has no right child" do
      assert is_nil(BinaryTreeZipper.right({[], BinaryTree.new(1, BinaryTree.new(1), nil)}))
    end
  end

  describe "top/1" do
    test "goes top from a current node to its parent", %{
      binary_tree_zipper: binary_tree_zipper
    } do
      assert binary_tree_zipper ==
               binary_tree_zipper
               |> BinaryTreeZipper.left()
               |> BinaryTreeZipper.top()

      assert binary_tree_zipper ==
               binary_tree_zipper
               |> BinaryTreeZipper.right()
               |> BinaryTreeZipper.top()
    end
  end

  describe "set_left_branch/2" do
    test "sets the left branch of a binary tree within the zipper to a value", %{
      binary_tree_zipper: binary_tree_zipper
    } do
      assert {_thread, binary_tree} =
               BinaryTreeZipper.set_left_branch(binary_tree_zipper, BinaryTree.new(1_000_000))

      assert BinaryTree.new(1_000_000) == binary_tree.left
    end

    test "sets the right branch of a binary tree within the zipper to a value", %{
      binary_tree_zipper: binary_tree_zipper
    } do
      assert {_thread, binary_tree} =
               BinaryTreeZipper.set_right_branch(binary_tree_zipper, BinaryTree.new(1_000_000))

      assert BinaryTree.new(1_000_000) == binary_tree.right
    end
  end
end
