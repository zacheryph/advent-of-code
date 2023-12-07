# frozen_string_literal: true
module Year2023
  class Day05 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data

      seeds.map do |seed|
        mappers.inject(seed) { |s, mapper| mapper.map(s) }
      end
      .min
    end

    def part_2
      data
      ranges = seeds.each_slice(2).map { |s| s[0]..(s[0]+s[1]) }

      mappers.each do |mapper|
        ranges = ranges.flat_map do |range|
          mapper.range(range)
        end.uniq
      end

      ranges.map(&:begin).min
    end

    private
      attr_reader :seeds, :mappers

      def data
        parts = input.split("\n\n")
        @seeds = parts.shift.split[1..].map(&:to_i)

        @mappers = parts.map do |p|
          p = p.split("\n")
          p.shift
          Mapper.new(p)
        end
      end

      class Mapper
        attr_accessor :mappings

        def initialize(rows)
          @mappings = rows.map do |r|
            Mapping.new(*r.split.map(&:to_i))
          end
        end

        def map(seed)
          mappings.lazy.map { |m| m.map(seed) }.reject(&:nil?).first || seed
        end

        def range(seed)
          uncovered = [seed]
          results = []

          mappings.each do |mapping|
            uncovered = uncovered.flat_map do |range|
              before, inside, after = mapping.range(range)
              results << inside if inside.size > 1
              [(before if before.size > 1), (after if after.size > 1)].compact
            end
          end

          [*uncovered, *results]
        end
      end

      class Mapping
        attr_reader :source, :offset

        def initialize(dest, source, length)
          @offset = source - dest
          @source = source..(source + length)
        end

        def map(seed)
          return nil unless source.include?(seed)
          seed - offset
        end

        def range(seed)
          before = seed.begin..(source.begin - 1).clamp(seed)
          inside = source.begin.clamp(seed)..source.end.clamp(seed)
          after = (source.end.clamp(seed) + 1)..seed.end

          [
            before,
            (inside.begin - offset)..(inside.end - offset),
            after
          ]
        end
      end
  end
end
