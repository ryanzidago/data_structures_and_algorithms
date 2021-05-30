defmodule BinarySearchTreeTest do
  use ExUnit.Case

  setup %{} do
    node = %TreeNode{
      val: 10,
      left: %TreeNode{
        val: 5,
        left: %TreeNode{
          val: 1
        }
      },
      right: %TreeNode{
        val: 15,
        right: %TreeNode{
          val: 20
        }
      }
    }

    {:ok, node: node}
  end

  describe "insert/2" do
    test "does not allow duplicate values" do
      node = TreeNode.new(3)
      assert node == BinarySearchTree.insert(node, 3)
    end

    test "if the inserted element is less than the current node, creates a new left child", %{
      node: expected
    } do
      node =
        TreeNode.new(10)
        |> BinarySearchTree.insert(15)
        |> BinarySearchTree.insert(5)
        |> BinarySearchTree.insert(20)
        |> BinarySearchTree.insert(1)

      assert node == expected
    end
  end

  describe "search/2" do
    test "if the BinarySearchTree contains the searched element, returns the node containing the element",
         %{node: node} do
      node_with_searched_value = BinarySearchTree.search(node, 1)
      expected = TreeNode.new(1)
      assert node_with_searched_value == expected
    end

    test "if the BinarySearchTree does not contains the element, returns `nil`", %{node: node} do
      assert is_nil(BinarySearchTree.search(node, 999))
    end
  end

  describe "leaf?/1" do
    test "returns `true` if the node has no descendants" do
      assert BinarySearchTree.leaf?(TreeNode.new())
    end

    test "returns `false` if the node has at least one descendant", %{node: node} do
      refute BinarySearchTree.leaf?(node)
    end
  end
end
