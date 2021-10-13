defmodule BinaryTreeTest do
  use ExUnit.Case

  setup %{} do
    symmetric_tree = %BinaryTree{
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

    asymmetric_tree = %BinaryTree{
      val: 1,
      left: %BinaryTree{
        val: 3,
        left: %BinaryTree{
          val: 3
        },
        right: %BinaryTree{
          val: 3
        }
      },
      right: %BinaryTree{
        val: 4,
        left: %BinaryTree{
          val: 4
        },
        right: %BinaryTree{
          val: 4
        }
      }
    }

    {
      :ok,
      symmetric_tree: symmetric_tree, asymmetric_tree: asymmetric_tree
    }
  end

  describe "new/{1,2,3}" do
    test "creates a new BinaryTree node" do
      assert %BinaryTree{val: 1} == BinaryTree.new(1)

      assert %BinaryTree{val: 1, left: %BinaryTree{val: 10}} ==
               BinaryTree.new(1, BinaryTree.new(10))

      assert %BinaryTree{val: 1, left: %BinaryTree{val: 10}, right: %BinaryTree{val: 100}} ==
               BinaryTree.new(1, BinaryTree.new(10), BinaryTree.new(100))
    end
  end

  describe "symmetric?/1" do
    test "returns `true` if the BinaryTree is symmetric", %{symmetric_tree: symmetric_tree} do
      assert BinaryTree.symmetric?(symmetric_tree)
    end

    test "returns `false` if the BinaryTree is asymmetric", %{asymmetric_tree: asymmetric_tree} do
      refute BinaryTree.symmetric?(asymmetric_tree)
    end
  end
end
