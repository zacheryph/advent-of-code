# frozen_string_literal: true
module Year2023
  class Day02 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    ZERO_CUBES = {"blue" => 0, "green" => 0, "red" => 0}.freeze
    PART_1_CUBES = {"blue" => 14, "green" => 13, "red" => 12}.freeze

    # 2377
    def part_1
      possible = data.select do |g, pulls|
        next if pulls.find do |pull|
          pull.find { |color, num| PART_1_CUBES[color] < num }
        end
        true
      end

      possible.keys.sum
    end

    # 71220
    def part_2
      minimums = data.values.map do |pulls|
        min = ZERO_CUBES.dup
        pulls.each do |pull|
          pull.each { |color, num| min[color] = num if num > min[color] }
        end
        min.values
      end
      minimums.map { |v| v.reduce(:*) }.sum
    end

    private
      # Processes each line of the input file and stores the result in the dataset
      def process_input(line)
        game, pulls = line.split(":")
        game = game.split.last.to_i
        pulls = pulls.split(";").map do |g|
          g.split(",").map {|x| x.split.reverse }.to_h.transform_values(&:to_i)
        end
        [game, pulls]
      end

      # Processes the dataset as a whole
      def process_dataset(set)
        set.to_h
      end
  end
end
