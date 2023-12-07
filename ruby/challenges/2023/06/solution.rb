# frozen_string_literal: true
module Year2023
  class Day06 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data
        .map { |d| Race.new(**d) }
        .map { |r| r.winning_holds.size }
        .reduce(:*)
    end

    def part_2
      time = data.map { |r| r[:time].to_s }.join.to_i
      distance = data.map { |r| r[:distance].to_s }.join.to_i

      Race.new(time:, distance:)
        .winning_holds
        .size
    end

    private

    # each line of raw input
    def process_input(line)
      title, numbers = line.split(":")

      [title.downcase.to_sym, numbers.split.map(&:to_i)]
    end

    # the whole dataset ran through process_input
    def process_dataset(set)
      set.to_h.then do |hash|
        hash.values.first.length.times.map do |n|
          {time: hash[:time][n], distance: hash[:distance][n]}
        end
      end
    end

    Race = Struct.new(:time, :distance, keyword_init: true) do
      def winning_holds
        ms = (1..time).to_a
        min = ms.find { |n| n * (time - n) > distance }
        max = ms.reverse.find { |n| n * (time - n) > distance }
        min..max
      end
    end
  end
end
