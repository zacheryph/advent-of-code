# frozen_string_literal: true
module Year2022
  class Day08 < Solution
    def part_1
      edge_count = ((tree_map.length + tree_map.first.length) * 2) - 4
      visible = Set.new

      counters = {
        rights: tree_map.length
      }

      rights = tree_map[1..-2].map.with_index do |row, idx|
        Counter.new(row.first).tap do |ctr|
          row[1..-2].each.with_index { |h, col| ctr.seen(h, [idx, col], visible) }
        end
      end

      lefts = tree_map[1..-2].map.with_index do |row, idx|
        Counter.new(row.last).tap do |ctr|
          row[1..-2].each.with_index.to_a.reverse.each { |h, col| ctr.seen(h, [idx, col], visible) }
        end
      end

      tops = tree_map.transpose[1..-2].map.with_index do |row, col|
        Counter.new(row.first).tap do |ctr|
          row[1..-2].each.with_index { |h, idx| ctr.seen(h, [idx, col], visible) }
        end
      end

      bottoms = tree_map.transpose[1..-2].map.with_index do |row, col|
        Counter.new(row.last).tap do |ctr|
          row[1..-2].each.with_index.to_a.reverse.each { |h, idx| ctr.seen(h, [idx, col], visible) }
        end
      end

      binding.pry

      edge_count + visible.count
    end

    def part_2
      range = (0..tree_map.length)
      tree_counts = {}

      tree_map.each.with_index do |row, row_idx|
        row.each.with_index do |col, col_idx|
          tree_counts[[row_idx, col_idx]] = visible_from(row_idx, col_idx)
        end
      end

      # binding.pry
      tree_counts.values.max
    end

    private

    def visible_from(row, col)
      height = tree_map[row][col]
      transposed_map = tree_map.transpose

      visible_count = ->(rows) {
        rows.reduce(0) do |c, tree|
          break c + 1 if tree >= height
          c + 1
        end
      }

      left   = visible_count.call(tree_map[row][...col].reverse)
      right  = visible_count.call(tree_map[row][(col + 1)..])
      top    = visible_count.call(transposed_map[col][...row].reverse)
      bottom = visible_count.call(transposed_map[col][(row +1)..])

      left * right * top * bottom
    end

    def visibility_counts(field, visible, reverse: false)
      field[1..-2].map.with_index do |row, idx|
        Counter.new(reverse ? row.last : row.first).tap do |ctr|
          enum = row[1..-2].each.with_index
          enum = enum.to_a.reverse if reverse

          enum.each { |h, col| ctr.seen(h, [idx, col], visible) }
        end
      end

    end

    class Counter
      attr_reader :count, :height

      def initialize(edge_height)
        @count = 0
        @height = edge_height
      end

      def seen(height, key, visible)
        if height > @height
          @height = height
          @count += 1
          visible.add(key)
        end
      end
    end

    def tree_map
      @map ||= input.split("\n").map { |l| l.split("").map(&:to_i) }
    end
  end
end
