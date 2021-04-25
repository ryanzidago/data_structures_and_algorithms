require 'set'

class City
  attr_accessor :name, :routes

  def initialize(name)
    @name = name
    @routes = {}
  end

  def add_route(city, price)
    @routes[city] = price
  end
end

# this implementation of Dijkstra's algorithm uses a hashset for simplicity
# instead of the commonly used priority queue
# (see `unvisited_cities` variable)
def dijkstra_shortest_path(starting_city, final_destination)
  cheapest_prices_table = {}
  cheapest_previous_stopover_city_table = {}
  unvisited_cities = Set[]
  visited_cities = {}

  cheapest_prices_table[starting_city.name] = 0
  current_city = starting_city

  while current_city
    visited_cities[current_city.name] = true
    unvisited_cities.delete(current_city)

    current_city.routes.each do |adjacent_city, price|
      unvisited_cities << adjacent_city unless visited_cities[adjacent_city.name]

      price_through_current_city = cheapest_prices_table[current_city.name] + price

      if !cheapest_prices_table[adjacent_city.name] || price_through_current_city < cheapest_prices_table[adjacent_city.name]
        cheapest_prices_table[adjacent_city.name] = price_through_current_city
        cheapest_previous_stopover_city_table[adjacent_city.name] = current_city.name
      end
    end

    current_city = unvisited_cities.min do |city|
      cheapest_prices_table[city.name]
    end
  end

  shortest_path = []
  current_city_name = final_destination.name

  while current_city_name != starting_city.name
    shortest_path << current_city_name
    current_city_name = cheapest_previous_stopover_city_table[current_city_name]
  end

  shortest_path << starting_city.name
  shortest_path = shortest_path.reverse!()

  {"shortest_path" => shortest_path, "cheapest_prices" => cheapest_prices_table}
end
