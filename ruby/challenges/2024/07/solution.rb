# frozen_string_literal: true
module Year2024
  class Day07 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data
        .select { |test| part_1_valid?(test.value, test.input[1..], value: test.input.first) }
        .map(&:value)
        .sum
    end

    def part_2
      nil
    end

    private

    OPS = [:*, :+].freeze

    Test = Data.define(:value, :input)

    def process_input(line)
      value, input = line.split(':')
      input = input.split.map(&:to_i)

      Test.new(value: value.to_i, input:)
    end

    def part_1_valid?(test, input, value:)
      return value == test if input.empty?


      OPS.each do |op|
        total = value.send(op, input[0])
        return true if part_1_valid?(test, input[1..], value: total)
      end
      return false
    end

    def part_2_valid?(test, input, value:)

    end
  end
end
