require 'pry'

class Vertex
  attr_accessor :value, :adjacent_vertices

  def initialize(value)
    @value = value
    @adjacent_vertices = {value => {value => 0 }}
  end

  def add_adjacent_vertex(adjacent_vertex)
    @adjacent_vertices[@value][adjacent_vertex.value] = 1
    adjacent_vertex.adjacent_vertices[adjacent_vertex.value][@value] = 0
  end
end

class Graph
  def dfs_traverse(starting_vertex, visited_vertices = {})
    visited_vertices[starting_vertex.value] = true

    vertex.adjacent_vertices[]
  end
end
