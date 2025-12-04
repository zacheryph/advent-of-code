# frozen_string_literal: true
module Year2025
  class Day03 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      joltages = data.map do |bank|
        batteries = [0] * 2
        bank.each { |b| batteries = joltage_shift(batteries, b) }
        batteries
      end

      joltages.map { |j| j.join.to_i }.sum
    end

    def part_2
      joltages = data.map do |bank|
        batteries = [0] * 12
        bank.each { |b| batteries = joltage_shift(batteries, b) }
        batteries
      end

      joltages.map { |j| j.join.to_i }.sum
    end

    private
      # Processes each line of the input file and stores the result in the dataset
      def process_input(line)
        line.split("").map(&:to_i)
      end

      def joltage_shift(batteries, current)
        batteries = [*batteries, current]
        (0..(batteries.length - 2)).each do |idx|
          if batteries[idx + 1] > batteries[idx]
            batteries = [*batteries[...idx], *batteries[(idx + 1)..], current]
            break
          end
        end

        batteries[...-1]
      end
    end
end
