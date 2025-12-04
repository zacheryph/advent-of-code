# frozen_string_literal: true
module Year2024
  class Day02 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data.select { |report| safe?(report) }.length
    end

    def part_2
      # binding.pry
      data
      .map { |report| filter(report) }
      .compact
      .length
    end

    private

    def process_input(line)
      line.split.map(&:to_i)
    end

    def safe?(report)
      return false if report != report.sort && report != report.sort.reverse
      return false if report.length != report.uniq.length

      report
        .each_cons(2)
        .all? { |a, b| (a-3..a+3).include?(b) }
    end

    # returns a cleaned up report or nil if its invalid
    def filter(report)
      return report if safe?(report)

      report
        .length
        .times
        .map { |i| report[0...i] + report[i+1..-1] }
        .find { |report| safe?(report) }
    end
  end
end
