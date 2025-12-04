# frozen_string_literal: true
module Year2024
  class Day01 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data.reduce(&:zip).map { |set| set.max - set.min }.sum
    end

    def part_2
      left, right = *data.map(&:tally)

      left.map { |k, v| k * v * right[k].to_i }.sum
    end

    private

    def process_input(line)
      line.split.map(&:to_i)
    end

    def process_dataset(set)
      set.transpose.map(&:sort)
    end
  end
end
