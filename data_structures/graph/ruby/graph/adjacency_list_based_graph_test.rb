require './adjacency_list_based_graph.rb'
require 'pry'
RSpec.describe Vertex do
  describe "#new" do
    it "creates a new Vertex with a value inside" do
      alice = Vertex.new("alice")


      expect(alice.value).to eq("alice")
    end
  end

  describe "#add_adjacent_vertex" do
    it "connects two vertices with each other" do
      alice = Vertex.new("alice")
      bob = Vertex.new("bob")

      alice.add_adjacent_vertex(bob)

      expect(alice.adjacent_vertices.include?(bob)).to be true
    end

    it "connects many vertices together" do
      alice = Vertex.new("alice")
      bob = Vertex.new("bob")
      cynthia = Vertex.new("cynthia")

      alice.add_adjacent_vertex(bob)
      alice.add_adjacent_vertex(cynthia)
      bob.add_adjacent_vertex(cynthia)
      cynthia.add_adjacent_vertex(bob)

      expect(alice.adjacent_vertices.include?(bob)).to be true
      expect(alice.adjacent_vertices.include?(cynthia)).to be true

      expect(bob.adjacent_vertices.include?(alice)).to be true
      expect(bob.adjacent_vertices.include?(cynthia)).to be true

      expect(cynthia.adjacent_vertices.include?(alice)).to be true
      expect(cynthia.adjacent_vertices.include?(bob)).to be true
    end
  end

  describe "#dfs_traverse" do
    it "traverses all vertices connected to a given vertex and return their values" do
      vertices = graph_factory()
      alice = vertices[0]
      bob = vertices[1]
      cynthia = vertices[2]
      visited_vertices = dfs_traverse(alice)

      expect(visited_vertices.length).to eq(3)

      expect(visited_vertices.include?(alice.value)).to be true
      expect(visited_vertices.include?(bob.value)).to be true
      expect(visited_vertices.include?(cynthia.value)).to be true
    end
  end

  describe "#dfs" do
    context "when the searched vertex is in the graph" do
      it "returns the searched vertex from a starting vertex" do
        alice = graph_factory()[0]
        cynthia = graph_factory()[2]

        maybe_alice = dfs(cynthia, "alice")

        expect(maybe_alice).to be_truthy
        expect(alice.value).to eq(maybe_alice.value)
      end
    end

    context "when the searched vertex is not in the graph" do
      it "returns `nil`" do
        alice = graph_factory()[0]
        expect(dfs(alice, "jean")).to be nil
      end
    end
  end

  describe "#bfs_traverse" do
    context "when the starting vertex is in the graph" do
      it "returned an array of all the connected verticies in the graph" do
        vertices = graph_factory()
        alice = vertices[0]
        bob = vertices[1]
        cynthia = vertices[2]

        visited_vertices = bfs_traverse(alice)

        expect(visited_vertices.include?(alice.value)).to be true
        expect(visited_vertices.include?(bob.value)).to be true
        expect(visited_vertices.include?(cynthia.value)).to be true
      end
    end
  end

  describe "#bfs" do
    context "when both starting vertex and searched value are in the graph" do
      it "returns the vertex containing the searched value" do
        vertices = graph_factory()
        alice = vertices[0]
        cynthia = vertices[2]

        maybe_alice = bfs(cynthia, "alice")
        expect(maybe_alice).to be_truthy
        expect(alice.value).to eq(maybe_alice.value)
      end
    end
  end
end

def graph_factory
  alice = Vertex.new("alice")
  bob = Vertex.new("bob")
  cynthia = Vertex.new("cynthia")

  alice.add_adjacent_vertex(bob)
  alice.add_adjacent_vertex(cynthia)
  bob.add_adjacent_vertex(cynthia)
  cynthia.add_adjacent_vertex(bob)

  [alice, bob, cynthia]
end
