require './weighted_graph.rb'

RSpec.describe City do
  describe "#add_route" do
    it "adds a new route (i.e. vertex) to the map (i.e. graph)" do
      paris = City.new("Paris")
      expect(paris.routes).to eq({})

      berlin = City.new("Berlin")
      paris.add_route(berlin, 175)

      expect(paris.routes).to eq({berlin => 175})

      milan = City.new("Milan")
      paris.add_route(milan, 200)

      expect(paris.routes).to eq({berlin => 175, milan => 200})
    end
  end

  describe "#dijkstra_shortest_path" do
    it "computes the shortest path from a starting city to a final destination" do
      weighted_graph = weighted_graph_factory()
      atlanta = weighted_graph["atlanta"]
      el_paso = weighted_graph["el_paso"]

      result = dijkstra_shortest_path(atlanta, el_paso)
      shortest_path = result["shortest_path"]
      cheapest_prices = result["cheapest_prices"]

      expect(shortest_path).to eq(["Atlanta", "Denver", "Chicago", "El Paso"])
      expect(cheapest_prices["El Paso"]).to eq(280)
    end
  end

  def weighted_graph_factory
    atlanta = City.new("Atlanta")
    boston = City.new("Boston")
    chicago = City.new("Chicago")
    denver = City.new("Denver")
    el_paso = City.new("El Paso")

    atlanta.add_route(boston, 100)
    atlanta.add_route(denver, 160)
    boston.add_route(chicago, 120)
    boston.add_route(denver, 180)
    chicago.add_route(el_paso, 80)
    denver.add_route(chicago, 40)
    denver.add_route(el_paso, 140)

    {"atlanta" => atlanta, "boston" => boston, "chicago" => chicago, "denver" => denver, "el_paso" => el_paso}
  end
end
