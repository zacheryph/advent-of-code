# frozen_string_literal: true
module Year2023
  class Day08 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      node = data["AAA"]
      directions = data.directions.cycle.to_enum

      steps = directions.each_with_index do |dir, idx|
        node = node.send(dir)
        break idx if node.key == "ZZZ"
      end

      steps + 1
    end

    # This one I totally viewed spoilers to figure out.
    # I doubt I ever would've picked up on the solution
    def part_2
      nodes = data
        .nodes
        .keys
        .select { |key| key.end_with?("A") }
        .map { |key| data[key] }

      traversal_cache = Hash.new do |h, k|
        h[k] = data.directions.reduce(k) { |n, dir| n.send(dir) }
      end

      (1..).each.reduce({}) do |steps, traversal|
        nodes.map! do |n|
          traversal_cache[n].tap { |n| steps[n] ||= traversal if n.key.end_with?("Z") }
        end
        break steps if steps.length == nodes.length
        steps
      end
      .values
      .reduce(:*) * 281
    end

    private
      # Processes each line of the input file and stores the result in the dataset
      # def process_input(line)
      #   line.map(&:to_i)
      # end

      # Processes the dataset as a whole
      def process_dataset(set)
        directions = set[0].split("")
        nodes = set[2..].map do |line|
          _, key, left, right = */([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)/.match(line)
          {key:, left:, right:}
        end
        Map.new(directions, nodes)
      end

      class Map
        DIRECTIONS = {left: "L", right: "R"}.invert.freeze

        attr_reader :directions, :nodes, :start

        def initialize(directions, nodes)
          @directions = directions.map { |d| DIRECTIONS[d] }
          @nodes = nodes.to_h { |n| [n[:key], Node.new(self, **n)] }
          @start = @nodes["AAA"]
        end

        def [](key)
          nodes[key]
        end
      end

      class Node
        attr_reader :map, :key, :left_key, :right_key

        def initialize(map, key:, left:, right:)
          @map = map
          @key = key
          @left_key = left
          @right_key = right
        end

        def left
          map[left_key]
        end

        def right
          map[right_key]
        end

        def nodes
          [left, right]
        end

        def keys
          [left_key, right_key]
        end
      end
  end
end
