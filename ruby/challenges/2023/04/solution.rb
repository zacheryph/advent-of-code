# frozen_string_literal: true
module Year2023
  class Day04 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data
        .map { |n| n[:winners].intersection(n[:scratches]) }
        .map do |n|
          case n.length
          when 0
            0
          else
            2**(n.length - 1)
          end
        end
        .sum
    end

    def part_2
      cards = Hash.new { |h, k| h[k] = 1 }
      data
        .each_with_index
        .map do |n, idx|
          cards[idx]
          extra = n[:winners].intersection(n[:scratches]).length
          extra.times { |n| cards[idx+1+n] += cards[idx] }
        end

      cards.values.sum
    end

    private
      # Processes each line of the input file and stores the result in the dataset
      def process_input(line)
        winners, scratches = line
          .gsub(/.*:/, "")
          .split("|")
          .map { |l| l.split.map(&:to_i) }

        {winners:, scratches:}
      end

      # Processes the dataset as a whole
      # def process_dataset(set)
      #   set
      # end
  end
end
