# frozen_string_literal: true
module Year2015
  class Day01 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    WALK = { "(" => +1, ")" => -1 }

    def part_1
      input
        .each_char
        .lazy
        .filter_map { |chr| WALK[chr] }
        .reduce(0) { |walk, step| step += walk }
    end

    def part_2
      loc = 0

      input
        .each_char
        .map { |chr| loc += (WALK[chr] || 0) }
        .index(-1) + 1
    end

    private
      # Processes each line of the input file and stores the result in the dataset
      # def process_input(line)
      #   line.map(&:to_i)
      # end

      # Processes the dataset as a whole
      # def process_dataset(set)
      #   set
      # end
  end
end
