# frozen_string_literal: true
module Year2022
  class Day01 < Solution
    def part_1
      elf_calories.first
    end

    def part_2
      elf_calories.take(3).sum
    end

    private

    def elf_calories
      @elf_calories =
        input.split("\n\n")
          .map { |elf| elf.split.map(&:to_i).sum }
          .sort
          .reverse
    end
  end
end
