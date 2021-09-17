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

  describe "symmetric?/1" do
    test "returns `true` if the BinaryTree is symmetric", %{symmetric_tree: symmetric_tree} do
      assert BinaryTree.symmetric?(symmetric_tree)
    end

    test "returns `false` if the BinaryTree is asymmetric", %{asymmetric_tree: asymmetric_tree} do
      refute BinaryTree.symmetric?(asymmetric_tree)
    end
  end
end
