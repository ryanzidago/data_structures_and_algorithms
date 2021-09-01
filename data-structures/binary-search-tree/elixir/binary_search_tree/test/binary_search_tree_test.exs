defmodule BinarySearchTreeTest do
  use ExUnit.Case

  #
  #        10
  #      5    15
  #     1       20
  #

  setup %{} do
    node = %BinarySearchTree{
      val: 10,
      left: %BinarySearchTree{
        val: 5,
        left: %BinarySearchTree{
          val: 1
        }
      },
      right: %BinarySearchTree{
        val: 15,
        right: %BinarySearchTree{
          val: 20
        }
      }
    }

    {:ok, node: node}
  end

  describe "new/1" do
    test "creates a new BST with the root node taking the value of the first argument" do
      assert %BinarySearchTree{val: 10} == BinarySearchTree.new(10)
    end
  end

  describe "insert/2" do
    test "does not allow duplicate values" do
      node = BinarySearchTree.new(3)
      assert node == BinarySearchTree.insert(node, 3)
    end

    test "if the inserted element is less than the current node, creates a new left child", %{
      node: expected
    } do
      node =
        BinarySearchTree.new(10)
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
      expected = BinarySearchTree.new(1)
      assert node_with_searched_value == expected
    end

    test "if the BinarySearchTree does not contains the element, returns `nil`", %{node: node} do
      assert is_nil(BinarySearchTree.search(node, 999))
    end
  end

  describe "leaf?/1" do
    test "returns `true` if the node has no descendants" do
      assert BinarySearchTree.leaf?(BinarySearchTree.new(0))
    end

    test "returns `false` if the node has at least one descendant", %{node: node} do
      refute BinarySearchTree.leaf?(node)
    end
  end

  describe "pre_order_traversal/1" do
    test "traverse pre order the BST and returns a list of the visited node", %{node: node} do
      assert [10, 5, 1, 15, 20] == BinarySearchTree.pre_order_traversal(node)
    end
  end

  describe "pre_order_traversal/2" do
    test "traverse pre order the BST and returns a list of the visited node, while applying the anonymous function to each visited node",
         %{node: node} do
      assert Enum.map([10, 5, 1, 15, 20], &(&1 * &1)) ==
               BinarySearchTree.pre_order_traversal(node, &(&1 * &1))
    end
  end

  describe "in_order_traversal/1" do
    test "traverse in order the BST and returns a list of the visited node", %{node: node} do
      assert [1, 5, 10, 15, 20] == BinarySearchTree.in_order_traversal(node)
    end
  end

  describe "in_order_traversal/2" do
    test "traverse in order the BST and returns a list of the visited node, while applying the anonymous function to each visited node",
         %{node: node} do
      assert Enum.map([1, 5, 10, 15, 20], &(&1 * &1)) ==
               BinarySearchTree.in_order_traversal(node, &(&1 * &1))
    end
  end

  describe "post_order_traversal/1" do
    test "traverse post order the BST and returns a list of the visited node", %{node: node} do
      assert [1, 5, 20, 15, 10] == BinarySearchTree.post_order_traversal(node)
    end
  end

  describe "post_order_traversal/2" do
    test "traverse post order the BST and returns a list of the visited node, while applying the anonymous function to each visited node",
         %{node: node} do
      assert Enum.map([1, 5, 20, 15, 10], &(&1 * &1)) ==
               BinarySearchTree.post_order_traversal(node, &(&1 * &1))
    end
  end

  describe "min/1" do
    test "returns the minimum value of the BST", %{node: node} do
      assert 1 == BinarySearchTree.min(node)
    end
  end

  describe "max/2" do
    test "returns the maximum value of the BST", %{node: node} do
      assert 20 == BinarySearchTree.max(node)
    end
  end

  describe "same_tree?/2" do
    test "returns `true` if both Binary Search Trees are the same", %{node: node} do
      node_1 = node
      node_2 = node

      assert BinarySearchTree.same_tree?(node_1, node_2)
    end

    test "otherwise, returns `false`", %{node: node} do
      refute BinarySearchTree.same_tree?(node, %BinarySearchTree{})
    end
  end

  describe "lowest_common_ancestor/3" do
    test "returns the lowest common ancestor for two nodes in a given tree", %{node: node} do
      node_1 = node.left
      node_2 = node.right

      assert node == BinarySearchTree.lowest_common_ancestor(node, node_1, node_2)

      node_1 = BinarySearchTree.search(node, 5)
      node_2 = BinarySearchTree.search(node, 15)

      assert node == BinarySearchTree.lowest_common_ancestor(node, node_1, node_2)
    end
  end
end
