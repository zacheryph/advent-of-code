# frozen_string_literal: true
module Year2024
  class Day08 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      antinodes { |l1, l2| resonant_offsets(l1, l2) }.length
    end

    def part_2
      antinodes { |l1, l2| harmonic_offsets(l1, l2) }.length
    end

    private

    def antinodes(&block)
      Set.new.tap do |antinodes|
        antennas.each do |freq, locations|
          locations.each_with_index do |loc, pos|
            locations[(pos + 1)..].each do |loc2|
              antinodes.merge(block.call(loc, loc2))
            end
          end
        end
      end
    end

    def within_map?(loc)
      return false if loc.any?(&:negative?)

      !data.dig(*loc).nil?
    end

    def resonant_offsets(antenna_1, antenna_2)
      diff = antenna_1.zip(antenna_2).map { _1.reduce(:-) }

      Enumerator.new do |yielder|
        [[antenna_1, :+], [antenna_2, :-]].each do |ant, offset|
          loc = ant.zip(diff).map { _1.reduce(offset) }
          yielder.yield(loc) if within_map?(loc)
        end
        nil
      end
    end

    def harmonic_offsets(antenna_1, antenna_2)
      diff = antenna_1.zip(antenna_2).map { _1.reduce(:-) }

      Enumerator.new do |yielder|
        [[antenna_1, :+], [antenna_2, :-]].flat_map do |ant, offset|
          loop do
            break unless within_map?(ant)
            yielder.yield(ant)
            ant = ant.zip(diff).map { _1.reduce(offset) }
          end
        end
        nil
      end
    end

    def antennas
      @antennas ||= begin
        res = Hash.new { |h, k| h[k] = [] }

        data.each_with_index do |line, row|
          line.each_with_index do |char, col|
            next if char == '.'
            res[char] << [row, col]
          end
        end
        res
      end
    end

    def process_input(line)
      line.split('')
    end
  end
end
