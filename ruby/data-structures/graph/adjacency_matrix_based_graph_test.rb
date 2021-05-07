require './adjacency_matrix_based_graph.rb'

RSpec.describe Vertex do
  describe "#new" do
    it "creates a new Vertex with a value and an initialized matrix" do
      alice = Vertex.new("alice")
      expect(alice.value).to eq("alice")
      expect(alice.adjacent_vertices["alice"]["alice"] = 0)
    end
  end

  describe "#add_adjacent_vertex" do
    it "creates an edge between two vertices" do
      alice = Vertex.new("alice")
      bob = Vertex.new("bob")

      alice.add_adjacent_vertex(bob)

      expect(alice.adjacent_vertices["alice"]["bob"]).to eq(1)
    end

    it "can create complex graph" do
      alice = Vertex.new("alice")
      bob = Vertex.new("bob")
      cynthia = Vertex.new("cynthia")

      alice.add_adjacent_vertex(bob)
      alice.add_adjacent_vertex(cynthia)
      bob.add_adjacent_vertex(cynthia)
      cynthia.add_adjacent_vertex(bob)

      puts()
      puts(alice.inspect())
      puts()

      puts(bob.inspect())
      puts()

      puts(cynthia.inspect())
    end
  end
end
