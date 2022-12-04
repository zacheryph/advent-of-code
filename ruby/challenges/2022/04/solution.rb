# frozen_string_literal: true
module Year2022
  class Day04 < Solution
    def part_1
      ranges
        .select { |elf1, elf2| elf1.cover?(elf2) || elf2.cover?(elf1) }
        .length
    end

    def part_2
      ranges
        .select do |elf1, elf2|
          elf1.cover?(elf2.begin) || elf1.cover?(elf2.end) || elf2.cover?(elf1.begin) || elf2.cover?(elf1.end)
        end
        .length
    end

    private

    def ranges
      input
        .split
        .map do |pairs|
          pairs.split(",").map { |r| Range.new(*r.split("-").map(&:to_i)) }
        end
    end
  end
end
