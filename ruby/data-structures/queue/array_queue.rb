class Queue
  def initialize
    @data = []
  end

  def enqueue(element)
    @data << element
  end

  def dequeue
    @data.shift()
  end

  def read
    @data.first()
  end
end
