defmodule TernaryTreeTest do
  use ExUnit.Case

  describe "new/1" do
    test "creates a new TernaryTree with the given parameters" do
      assert ternary_tree =
               TernaryTree.new(
                 val: 100,
                 left: nil,
                 mid: TernaryTree.new(),
                 right: TernaryTree.new(val: 50)
               )

      assert ternary_tree.val == 100
      assert ternary_tree.left == nil
      assert ternary_tree.mid == %TernaryTree{}
      assert ternary_tree.right == %TernaryTree{val: 50}
    end
  end
end
