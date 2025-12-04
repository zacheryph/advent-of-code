# frozen_string_literal: true
module Year2023
  class Day10 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      tails = [start, start].zip(connected(start))

      steps = (1..).each.map do |step|
        tails.each do |tail|
          binding.pry if tail.last.nil?
          tail.push((connected(tail.last) - [tail[-2]]).first)
        end
        printf "#{step} "
        break if step > 1000
        # break if tails[0].include?(tails[1].last) || tails[1].include?(tails[0].last)
      end
      binding.pry
      steps
    end

    def part_2
      nil
    end

    private

    VALUES = {
      "|" => %w(U D),
      "-" => %w(L R),
      "L" => %w(U R),
      "J" => %w(U L),
      "F" => %w(D R),
      "7" => %w(D L),
      "." => %w(),
      "S" => %w(U D L R)
    }

    SHIFT = {
      "U" => [-1,  0],
      "D" => [ 1,  0],
      "R" => [ 0,  1],
      "L" => [ 0, -1],
    }

    OPPOSITES = {
      "U" => "D",
      "D" => "U",
      "R" => "L",
      "L" => "R",
    }

    def start
      data.each_with_index do |line, row|
        line.each_with_index do |c, col|
          return [row, col] if c == "S"
        end
      end
    end

    # [[r,c], ...]
    def connected(spot)
      SHIFT.map do |key, shift|
        peek = spot.zip(shift).map(&:sum) rescue binding.pry
        next if peek.include?(-1)
        binding.pry if VALUES[data.dig(*peek)] == nil
        peek if VALUES[data.dig(*peek)].include?(OPPOSITES[key])
      end.compact
    end

    def process_input(line)
      line.split("")
    end
      # Processes each line of the input file and stores the result in the dataset
      # def process_input(line)
      #   line.map(&:to_i)
      # end

      # Processes the dataset as a whole
      # def process_dataset(set)
      #   set
      # end
  end
end
