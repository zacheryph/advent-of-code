# frozen_string_literal: true
module Year2024
  class Day03 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      input
        .scan(/mul\((\d+),(\d+)\)/)
        .map { _1.to_i * _2.to_i }
        .sum
    end

    Context = Struct.new(:enabled, :total, keyword_init: true)

    def part_2
      input
        .scan(/(mul|don't\(\)|do\(\))(?:\((\d+),(\d+)\))?/)
        .each
        .with_object({enabled: true, total: 0}) do |step, ctx|
          case step
            in ["do()", *] then ctx[:enabled] = true
            in ["don't()", *] then ctx[:enabled] = false
            in ["mul", a, b] then ctx[:total] += a.to_i * b.to_i if ctx[:enabled]
            else raise "WTF? #{ctx}"
          end
        end
        .fetch(:total)
    end
  end
end
