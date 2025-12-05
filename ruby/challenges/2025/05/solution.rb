# frozen_string_literal: true
module Year2025
  class Day05 < Solution
    def part_1
      available_ids.count do |id|
        combined_ranges.any? { it.cover?(id) }
      end
    end

    def part_2
      sorted_ranges = fresh_ranges.sort_by(&:begin)
      ranges = [sorted_ranges.first]

      sorted_ranges.each do |r|
        prev = ranges.last
        if prev.end >= r.begin
          ranges[-1] = Range.new(prev.begin, [prev.end, r.end].max)
        else
          ranges << r
        end
      end

      ranges.map(&:count).sum
    end

    private

    def data
      @data ||= input.split("\n\n")
    end

    def available_ids
      @available_ids ||= data.last.split.map(&:to_i)
    end

    def fresh_ranges
      @fresh_ranges ||= data
        .first
        .split
        .map { Range.new(*it.split("-").map(&:to_i)) }
        .sort_by(&:first)
    end

    def combined_ranges
      @combined_ranges ||= fresh_ranges.each_with_object([fresh_ranges.first]) do |r, ranges|
        prev = ranges.last

        if prev.end < r.begin
          ranges << r
        else
          ranges[-1] = Range.new(prev.begin, [prev.end, r.end].max)
        end
      end
    end
  end
end
