# frozen_string_literal: true
module Year2022
  class Day06 < Solution
    def part_1
      start_of_packet(4)
    end

    def part_2
      start_of_packet(14)
    end

    private

    def start_of_packet(length)
      offset = length - 1

      (buffer.length - offset).times do |idx|
        next if buffer[idx..idx+offset].uniq.length < length
        return idx + length
      end
    end

    def buffer
      @buffer ||= input.each_char.map(&:ord)
    end
  end
end
