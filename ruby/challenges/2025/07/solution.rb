# frozen_string_literal: true
module Year2025
  class Day07 < Solution
    def part_1
      beams = [start_beam]
      splitters = 0

      matrix[1..].each do |row|
        beams = beams.map do |beam|
          next beam unless row[beam] == "^"
          splitters += 1
          [beam - 1, beam + 1]
        end
        .flatten
        .uniq
      end

      splitters
    end

    def part_2
      timelines = { start_beam => 1 }

      matrix[1..].each do |row|
        new_timelines = Hash.new { |h, k| h[k] = 0 }

        timelines.each do |beam, lines|
          if row[beam] == "^"
            new_timelines[beam - 1] += lines
            new_timelines[beam + 1] += lines
          else
            new_timelines[beam] += lines
          end
        end

        timelines = new_timelines
      end

      timelines.values.sum
    end

    private

    def start_beam
      matrix.first.index("S")
    end

    def matrix
      @matrix ||= input.split("\n").map { it.split("") }
    end
  end
end
