defmodule StackTest do
  use ExUnit.Case

  describe "new/0" do
    test "creates a new stack" do
      assert %Stack{data: []} == Stack.new()
    end
  end

  describe "new/1" do
    test "creates a new stack, with the given data" do
      assert %Stack{data: [1, 2, 3]} == Stack.new([1, 2, 3])
    end
  end

  describe "push/2" do
    test "pushes an element on to the top of stack" do
      outcome =
        Stack.new()
        |> Stack.push(3)
        |> Stack.push(2)
        |> Stack.push(1)

      assert %Stack{data: [1, 2, 3]} == outcome
    end
  end

  describe "pop/1" do
    test "pops the top element of the stack" do
      outcome =
        [1, 2, 3]
        |> Stack.new()
        |> Stack.pop()

      assert {1, %Stack{data: [2, 3]}} == outcome
    end

    test "returns `{nil, stack}` if the stack is empty" do
      assert {nil, %Stack{data: []}} = Stack.pop(Stack.new())
    end
  end
end
