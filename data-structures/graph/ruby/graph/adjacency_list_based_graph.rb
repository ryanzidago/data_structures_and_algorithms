class Vertex
  attr_accessor :value, :adjacent_vertices

  def initialize(value)
    @value = value
    @adjacent_vertices = []
  end

  def add_adjacent_vertex(vertex)
    return self if @adjacent_vertices.include?(vertex)
    @adjacent_vertices << vertex
    vertex.add_adjacent_vertex(self)
  end
end

def dfs_traverse(vertex, visited_vertices = {})
  visited_vertices[vertex.value] = true

  vertex.adjacent_vertices.each do |adjacent_vertex|
    next if visited_vertices[adjacent_vertex.value]
    dfs_traverse(adjacent_vertex, visited_vertices)
  end

  return visited_vertices.keys
end

def dfs(vertex, search_value, visited_vertices = {})
  return vertex if vertex.value == search_value
  visited_vertices[vertex.value] = true

  vertex.adjacent_vertices.each do |adjacent_vertex|
    next if visited_vertices[adjacent_vertex.value]
    return adjacent_vertex if adjacent_vertex.value == search_value
    vertex_we_are_searching_for = dfs(adjacent_vertex, search_value, visited_vertices)
    return vertex_we_are_searching_for if vertex_we_are_searching_for
  end
  return nil
end

def bfs_traverse(starting_vertex)
  queue = Queue.new()
  visited_vertices = {}
  visited_vertices[starting_vertex.value] = true
  queue << starting_vertex

  while !queue.empty?
    current_vertex = queue.pop()
    current_vertex.adjacent_vertices.each do |adjacent_vertex|
      if !visited_vertices[adjacent_vertex.value]
        visited_vertices[adjacent_vertex.value] = true
        queue << adjacent_vertex
      end
    end
  end

  visited_vertices.keys
end

def bfs(starting_vertex, search_value)
  queue = Queue.new()
  visited_vertices = {}
  visited_vertices[starting_vertex.value] = true
  queue << starting_vertex

  while !queue.empty?
    current_vertex = queue.pop()
    return current_vertex if current_vertex.value == search_value

    current_vertex.adjacent_vertices.each do |adjacent_vertex|
      if !visited_vertices[adjacent_vertex.value]
        visited_vertices[adjacent_vertex.value] = true
        queue << adjacent_vertex
      end
    end
  end
end
