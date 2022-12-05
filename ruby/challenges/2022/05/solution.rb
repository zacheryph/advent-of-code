# frozen_string_literal: true
module Year2022
  class Day05 < Solution
    def part_1
      commands.each do |cmd|
        cmd.count.times do
          yard.move(cmd.from, cmd.to)
        end
      end

      yard.tops
    end

    def part_2
      commands.each do |cmd|
        yard.move(cmd.from, cmd.to, count: cmd.count)
      end

      yard.tops
    end

    private

    Command = Struct.new(:count, :from, :to, keyword_init: true) do
      TEMPLATE = /move (?<count>\d+) from (?<from>\d+) to (?<to>\d+)/

      def self.parse(line)
        match = TEMPLATE.match(line)
        new(count: match[:count].to_i, from: match[:from].to_i - 1, to: match[:to].to_i - 1)
      end
    end

    class Yard
      attr_accessor :stacks

      def initialize(input)
        @stacks =
          input
            .each_line
            .map do |line|
              line
                .each_char
                .each_slice(4)
                .map { |crate| crate.join.gsub(/[^A-Z]/, "").strip }
            end
            .reverse
            .transpose
            .map { |stack| stack.reject(&:empty?) }
      end

      def move(from, to, count: 1)
        stacks[to].push(*stacks[from].pop(count))
      end

      def tops
        stacks.map(&:last).join
      end
    end

    def yard
      @yard ||= Yard.new(parse_input.first)
    end

    def commands
      @commands ||= parse_input.last.each_line.map { |cmd| Command.parse(cmd) }
    end

    def parse_input
      input.split("\n\n")
    end
  end
end
