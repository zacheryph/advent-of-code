# frozen_string_literal: true
module Year2022
  class Day11 < Solution
    def part_1
      pack = monkeys(worry: 3)
      20.times { pack.each(&:turn) }
      pack.map(&:inspections).sort[-2..].reduce(:*)
    end

    def part_2
      pack = monkeys
      10000.times { |i| pack.each(&:turn) }
      pack.map(&:inspections).sort[-2..].reduce(:*)
    end

    private

    def monkeys(worry: nil)
      pack = input.split("\n\n").map { |set| Monkey.new(set, worry:) }
      denominator = pack.map(&:divisor).reduce(:*)
      pack.each do |monkey|
        monkey.denominator = denominator
        monkey.pack = pack
      end
    end

    class Operation
      attr_accessor :operator, :values

      def initialize(op)
        match = /new = ((?:old)|\d+) (.) ((?:old)|\d+)/.match(op).to_a[1..]
        @operator = match.delete_at(1).to_sym
        @values = match.map do |v|
          case v
          in "old"
            :old
          else
            v.to_i
          end
        end
      end

      def call(old)
        values.map { |v| v == :old ? old : v }.reduce(operator)
      end
    end

    class Monkey
      attr_accessor :items, :pack, :denominator
      attr_reader :operation, :divisor, :on_true, :on_false
      attr_reader :worry, :inspections

      def initialize(input, worry: nil)
        @inspections = 0
        @worry = worry

        input.split("\n").each do |l|
          op, val = l.split(":").map(&:strip)
          case op
          when "Starting items" #: 98, 89, 52
            @items = val.split(",").map(&:strip).map(&:to_i)
          when "Operation" #: new = old * 2
            @operation = Operation.new(val)
          when "Test" #: divisible by 5
            @divisor = val.split.last.to_i
          when "If true" #: throw to monkey 6
            @on_true = val.split.last.to_i
          when "If false" #: throw to monkey 1
            @on_false = val.split.last.to_i
          end
        end
      end

      def turn
        @inspections += items.length
        items.each do |item|
          item = operation.call(item)
          item = worry ? item / worry : item % denominator
          if (item % divisor) == 0
            pack[on_true].items << item
          else
            pack[on_false].items << item
          end
        end
        @items = []
      end
    end
  end
end
