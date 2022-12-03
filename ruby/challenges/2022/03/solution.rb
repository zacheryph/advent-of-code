# frozen_string_literal: true
module Year2022
  class Day03 < Solution
    def part_1
      rucksacks
        .map { |sacks| sacks.inject(:&).sum }
        .sum
    end

    def part_2
      rucksacks
        .each_slice(3).map do |group|
          group.map(&:flatten).inject(:&).sum
        end
        .sum
    end

    private

    def rucksacks
      input
        .split
        .map { |sack| sack.each_char.map { |item| priority[item] } }
        .map do |sack|
          split = sack.length / 2
          [sack[...split], sack[split..]]
        end
    end

    def priority
      @priority ||= begin
        lower = ('a'..'z').each_with_index.to_h { |c, idx| [c, idx + 1] }
        upper = ('A'..'Z').each_with_index.to_h { |c, idx| [c, idx + 27] }
        lower.merge(upper)
      end
    end
  end
end
