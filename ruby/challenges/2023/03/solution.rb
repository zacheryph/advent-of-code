# frozen_string_literal: true
module Year2023
  class Day03 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data.to_enum.with_index.flat_map do |line, row|
        line.numbers.map do |col, n|
          n[:value] if marked?(row, col, n[:length])
        end
      end
      .compact
      .sum
    end

    def part_2
      res = data.to_enum.with_index.flat_map do |line, row|
        line.markers.select { |k, v| v == "*" }.map do |col, _|
          nums = touching(row, col)
          nums if nums.length == 2
        end
      end
      .compact
      .map { |p| p.reduce(:*) }
      .sum
    end

    private

    def marked?(row, col, length = nil)
      length ||= data[row].numbers[col][:length]


      rows = ((row - 1).clamp(0..))..(row + 1)
      cols = ((col - 1).clamp(0..))..(col + length)
      rows.any? { |r| data[r]&.marked?(cols) }
    end

    def touching(row, col)
      rows = ((row - 1).clamp(0..))..(row + 1)
      cols = ((col - 1).clamp(0..))..(col + 1)

      rows.flat_map do |r|
        data[r].numbers.select do |idx, n|
          cols.include?(idx) || cols.include?(idx + n[:length] - 1)
        end
      end
      .map(&:values)
      .flatten
      .map { |num| num[:value] }
    end

    def process_input(line)
      offset = 0
      numbers = line
        .gsub(/[^0-9]+/, " ")
        .split
        .to_h do |n|
          idx = line.index(n, offset)
          offset = idx + n.length

          [idx, {length: n.length, value: n.to_i}]
        end

      markers = line
        .gsub(/[0-9.]/, " ")
        .each_char
        .with_index
        .reject { |c, _| c == " " }
        .to_h { |c, idx| [idx, c] }

      {numbers:, markers:}
    end

    def process_dataset(set)
      set.map { |line| Line.new(**line) }
    end

    Line = Struct.new(:numbers, :markers, keyword_init: true) do
      def marked?(range)
        range = (range.begin.clamp(0..))..(range.end)
        markers.keys.any? { |m| range.include?(m) }
      end

      def number(col)
        numbers[col]
      end
    end
  end
end
