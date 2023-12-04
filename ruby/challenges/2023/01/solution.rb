# frozen_string_literal: true
module Year2023
  class Day01 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    # 54338
    def part_1
      data.map do |line|
        nums = line.gsub(/[^0-9]/, "").chars
        [nums.first, nums.last].join.to_i
      end.sum
    end

    NUM_WORDS = [
      "zero",
      "one",
      "two",
      "three",
      "four",
      "five",
      "six",
      "seven",
      "eight",
      "nine"
    ]
    NUM_VALUE = (("0".."9").to_enum.with_index + NUM_WORDS.to_enum.with_index).to_h
    NUM_MATCH = /([0-9]|#{NUM_WORDS.join("|")})/

    REVERSE_WORDS = NUM_WORDS.map(&:reverse)
    REVERSE_NUM_MATCH = /([0-9]|#{REVERSE_WORDS.join("|")})/

    # 53389
    def part_2
      data.map do |line|
        [NUM_VALUE[line.match(NUM_MATCH).to_s], NUM_VALUE[line.reverse.match(REVERSE_NUM_MATCH).to_s.reverse]]
      end.map { |l| l.map { |n| NUM_WORDS.index(n) || n }.join.to_i }.sum
    end
  end
end
