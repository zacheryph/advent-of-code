# frozen_string_literal: true
module Year2024
  class Day04 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data
        .each
        .with_index
        .flat_map do |line, row|
          line
            .each_char
            .with_index
            .select { |chr, _| chr == "X" }
            .map { |_, col| match_xmas([row, col]) }
        end
        .sum
    end

    def part_2
      data
        .each
        .with_index
        .flat_map do |line, row|
          line
            .each_char
            .with_index
            .select { |chr, _| chr == "A" }
            .select { |_, col| match_x([row, col]) }
        end
        .length
    end

    private

    PART_2_WORDS = [%w[M A S], %w[S A M]]
    PART_2_PATHS = {
      [-1, -1] => [1, 1],
      [-1, 1] => [1, -1],
    }

    def match_x(loc)
      PART_2_PATHS.all? do |off, dir|
        PART_2_WORDS.include?(spell(loc.zip(off).map(&:sum), dir, 3))
      end
    end

    def process_input(line)
      ".#{line}."
    end

    def process_dataset(set)
      [".", *set, "." ]
    end

    PART_1_WORD = %w[X M A S]

    # [row, col]
    PART_1_DIRECTIONS = [
      [-1, -1], [-1, 0], [-1, 1],
      [ 0, -1],          [ 0, 1],
      [ 1, -1], [ 1, 0], [ 1, 1],
    ]

    def match_xmas(loc)
      return 0 unless index(loc) == 'X'

      PART_1_DIRECTIONS
        .select { |dir| spell(loc, dir, 4) == PART_1_WORD }
        .length
    end

    EMPTY_STR = ""
    def index(loc)
      (data[loc.first] || EMPTY_STR)[loc.last] || EMPTY_STR
    end

    def spell(loc, dir, len)
      len.times.map do
        index(loc).tap { loc = loc.zip(dir).map(&:sum) }
      end
    end

    def match?(loc, dir, match)
      spell = match.length.times.map do
        index(loc).tap do
          loc = loc.zip(dir).map(&:sum)
        end
      end

      spell == match.split
    end
  end
end
