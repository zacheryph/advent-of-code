# frozen_string_literal: true
module Year2022
  class Day10 < Solution
    def part_1
      signal_cycles = [20, 60, 100, 140, 180, 220]
      signal_strengths = []

      vm = VM.new do |tick, _, value|
        next unless signal_cycles.include?(tick)

        signal_strengths << tick * value
      end

      instructions.each { |inst| vm.op(*inst) }
      signal_strengths.sum
    end

    def part_2
      vm = VM.new do |tick, value|
        puts if (tick) % 40 == 0

        visible = (value-1)..(value+1)
        pixel = visible.cover?(tick % 40) ? "#" : "."
        printf pixel
      end

      instructions.each { |inst| vm.op(*inst) }
      puts "\nSUBMIT MANUALLY"
    end

    private

    def instructions
      input.each_line.map { |l| l.split }
    end

    class VM
      attr_reader :tick, :callback, :value

      def initialize(&block)
        @tick = 0
        @value = 1
        @callback = block

        callback.call(tick, value, value)
      end

      def cycle
        @tick += 1
        before = @value
        yield if block_given?
        callback.call(tick, value, before)
      end

      def op(op, arg = nil)
        case op.downcase
        when "noop"
          cycle
        when "addx"
          cycle
          cycle { @value += arg.to_i }
        end
      end
    end
  end
end
