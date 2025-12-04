# frozen_string_literal: true
module Year2023
  class Day15 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data.map(&method(:hash)).sum
    end

    def part_2
      # hash => {label => focal_length}
      boxes = Hash.new { |h, k| h[k] = Hash.new({}) }
      data.each do |seq|
        _, label, op, length = *seq.match(/([^=-]+)([=-])([0-9]+)?/)
        case op
        when "="
          boxes[hash(label)][label] = length
        when "-"
          boxes[hash(label)].delete(label)
        end
      end

      boxes.flat_map do |box, labels|
        labels.each_with_index.map do |(_, length), idx|
          (box + 1) * (idx + 1) * length.to_i
        end
      end.sum
    end

    def hash(str)
      str.each_char.map(&:ord).reduce(0) { |hash, char| (hash + char) * 17 % 256 }
    end

    private
      # Processes each line of the input file and stores the result in the dataset
      def process_input(line)
        line.split(",")
      end

      # Processes the dataset as a whole
      # def process_dataset(set)
      #   set.split(/[,\n]/)
      # end
  end
end
