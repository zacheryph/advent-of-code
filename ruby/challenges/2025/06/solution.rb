# frozen_string_literal: true
module Year2025
  class Day06 < Solution
    def part_1
      input
        .split("\n")
        .map(&:split)
        .then { it.first.zip(*it[1..]) }
        .map { it[...-1].map(&:to_i).reduce(it.last.to_sym) }
        .sum
    end

    def part_2
      map = input.split("\n").map { it.split("") }
      ops = map.last.map(&:strip).delete_if(&:empty?).map(&:to_sym).reverse

      ([""] * map.map(&:length).max)
        .zip(*map[...-1])
        .reverse
        .each_with_object([[]]) do |num, acc|
          n = num.map(&:strip).delete_if(&:empty?).join
          n.empty? ? (acc << []) : acc.last << n
        end
        .zip(ops)
        .map { |numbers, op| numbers.map(&:to_i).reduce(op) }
        .sum
    end
  end
end
