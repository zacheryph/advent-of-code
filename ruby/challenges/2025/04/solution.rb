# frozen_string_literal: true
module Year2025
  class Day04 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      walk_map(false)
    end

    def part_2
      accessible = 0

      while (now = walk_map(true)) > 0
        accessible += now
      end

      accessible
    end

    def walk_map(remove)
      accessible = 0

      # its save to skip the first row
      ridx = 0
      cidx = 0
      row_count = paper_map.length - 1
      col_count = paper_map.first.length - 1

      while ridx < row_count
        ridx += 1
        cidx = 0
        while cidx < col_count
          cidx += 1

          next unless paper_map[ridx][cidx] == "@"

          # binding.pry
          next if paper_map[(ridx-1)..(ridx+1)].map { |r| r[(cidx-1)..(cidx+1)] }.flatten.tally["@"] > 4

          accessible += 1
          paper_map[ridx][cidx] = "." if remove
        end
      end

      accessible
    end

    private
      # i cheat to make things easier by adding a buffer. its not
      # as performant but it makes the code easier to follow :)
      def paper_map
        @paper_map ||= [
          ["."] * (data.first.length + 2),
          *data.map { |r| ["."] + r + ["."] },
          ["."] * (data.first.length + 2),
        ]

      end
      # Processes each line of the input file and stores the result in the dataset
      def process_input(line)
        line.split("")
      end
  end
end
