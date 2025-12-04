# frozen_string_literal: true
module Year2024
  class Day05 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      updates
        .select { |update| valid_order?(update) }
        .map { |pages| pages[(pages.length - 1) / 2] }
        .sum
    end

    def part_2
      updates
        .reject { |update| valid_order?(update) }
        .map do |update|
          [].tap do |result|
            until update.empty?
              page = update.find { |p| ((rules[p] || []) & update).empty? }
              result.push(update.delete(page))
            end
          end
        end
        .map { |pages| pages[(pages.length - 1) / 2] }
        .sum
    end

    private

    def valid_order?(pages)
      pages = pages.reverse

      pages.each.with_index.all? do |page, idx|
        (rules[page] || []).all? do |dependent|
          (pages.index(dependent) || idx) >= idx
        end
      end
    end

    def rules
      @rules ||=
        input
          .split("\n\n")
          .first
          .split("\n")
          .map { |rule| rule.split("|").map(&:to_i) }
          .group_by(&:pop)
          .transform_values(&:flatten)
    end

    def updates
      @updates ||=
        input
          .split("\n\n")
          .last
          .split("\n")
          .map { |update| update.split(",").map(&:to_i) }
    end
  end
end
