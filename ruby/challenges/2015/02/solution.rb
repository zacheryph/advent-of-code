# frozen_string_literal: true
module Year2015
  class Day02 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data
        .map do |data|
          data => {l:, w:, h:}
          single_sides = [l*w, w*h, h*l]
          single_sides.min + single_sides.sum { |n| n*2 }
        end
        .sum
    end

    def part_2
      data
        .map do |data|
          data => {l:, w:, h:}
          ([l, w, h].sort.take(2) * 2).sum + (l*w*h)
        end
        .sum
    end

    private

    DIMENSIONS = %i[l w h]

    def process_input(line)
      DIMENSIONS.zip(line.split("x").map(&:to_i)).to_h
    end
  end
end
