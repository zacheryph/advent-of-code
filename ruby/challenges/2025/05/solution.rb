# frozen_string_literal: true
module Year2025
  class Day05 < Solution
    def part_1
      available_ids.count { |id| fresh_ingredient?(id) }
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
      def fresh_ingredient?(id)
        fresh_ranges
          .any? { |r| r.include?(id) }
      end

      def fresh_ranges
        @fresh_ranges ||= input
          .split("\n\n")
          .first
          .split
          .map do |split|
            Range.new(*split.split("-").map(&:to_i))
          end
      end

      def available_ids
        @available_ids ||= input.split("\n\n").last.split.map(&:to_i)
      end
    end
end
