# frozen_string_literal: true
module Year2023
  class Day09 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data.map { |hist| next_value(hist) }.sum
    end

    def part_2
      data
        .each { |hist| hist[0].reverse! }
        .map { |hist| next_value(hist) }
        .sum
    end

    private

    def next_value(hist)
      row = (1..).each do |n|
        break n if hist[n].uniq.length == 1
      end
      hist[row] << hist[row].last
      (row - 1).downto(0).each do |n|
        hist[n] << hist[n].last + hist[n + 1].last
      end

      hist[0].last
    end

    # Processes each line of the input file and stores the result in the dataset
    def process_input(line)
      line.split.map(&:to_i)
    end

    # Processes the dataset as a whole
    def process_dataset(set)
      set.map do |hist|
        Hash.new do |h, k|
          h[k] = h[k - 1].each_cons(2).map { |y, x| x - y }
        end.tap { |h| h[0] = hist }
      end
    end
  end
end
