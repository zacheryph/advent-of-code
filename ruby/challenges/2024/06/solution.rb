# frozen_string_literal: true
module Year2024
  class Day06 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      route.uniq.length
    end

    def part_2
      steps = route.uniq - [start_position]
      loops = Set.new
      total = steps.length

      puts
      res = steps[1..].each_with_index do |loc, idx|
        printf "\rTests Remain: #{total - idx} (#{loops.length})"
        data[loc.first][loc.last] = '#'
        loops.add(loc) if route.nil?
        data[loc.first][loc.last] = '.'
      end
      puts
      # binding.pry
      loops.length
    end

    private

    OBSTACLE = '#'

    # [row, col]; we start going "north"
    DIRECTION = [
      [-1,  0],
      [ 0,  1],
      [ 1,  0],
      [ 0, -1],
    ].freeze

    def start_position
      data.each_with_index do |line, row|
        next if (col = line.index('^')).nil?

        return [row, col]
      end
    end

    def peek(loc, dir)
      loc = loc.zip(dir).map(&:sum)

      [loc, loc.any?(:negative?) ? nil : data.dig(*loc)]
    rescue
      binding.pry
    end

    def route
      steps = [start_position]
      turns = []
      path = DIRECTION.cycle
      dir = path.next

      loop do
        current = steps.last
        # return nil if turns.include?([current, dir]) # we looped

        loc, step = peek(current, dir)
        return steps if step.nil? # we left the party

        # yield(loc, dir, step == '#' ? '#' : nil) if block_given?

        # next step is an obstacle, so we turn and try again
        if OBSTACLE == step
          turns << [current, dir]
          dir = path.next
          next
        end

        steps << loc
      end
    end

    def process_input(line)
      line.split('')
    end
  end
end
