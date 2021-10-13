defmodule QueueTest do
  use ExUnit.Case

  describe "new/0" do
    test "creates a queue" do
      assert %Queue{data: {[], []}} == Queue.new()
    end
  end

  describe "new/1" do
    test "creates a queue from a list" do
      assert %Queue{data: {[1, 2, 3], []}} == Queue.new([1, 2, 3])
    end

    test "creates a queue from a range" do
      assert %Queue{data: {[1, 2, 3], []}} == Queue.new(1..3)
    end
  end

  describe "push/2" do
    test "pushes an item onto the queue" do
      assert queue = %Queue{data: {[1], []}} = Queue.push(Queue.new(), 1)
      assert _queue = %Queue{data: {[2, 1], []}} == Queue.push(queue, 2)
    end
  end

  describe "pop/2" do
    test "pops an item from the queue, returning a two element tuple where the first element is the item and the second element is the queue without the popped item" do
      assert {3, queue = %Queue{data: {[], [2, 1]}}} = Queue.pop(Queue.new(1..3))
      assert {2, queue = %Queue{data: {[], [1]}}} = Queue.pop(queue)
      assert {1, _queue = %Queue{data: {[], []}}} = Queue.pop(queue)
    end

    test "returns `nil` when the queue is empty" do
      assert {nil, %Queue{data: {[], []}}} = Queue.pop(Queue.new())
    end
  end

  describe "empty?/1" do
    test "returns `true` when the queue is empty" do
      assert true == Queue.empty?(Queue.new())
    end

    test "returns `false` when the queue is not empty" do
      assert false == Queue.empty?(Queue.new(1..3))
    end
  end
end
