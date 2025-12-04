# frozen_string_literal: true
module Year2025
  class Day01 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      pos = START_POS
      stop_count = 0

      data.each do |motion|
        pos += motion
        stop_count += 1 if (pos % 100) == 0
      end

      stop_count
    end

    # 4367 TO LOW
    # 4879 TO LOW
    # 5318 TO LOW
    # 6556 wrong
    def part_2
      pos = START_POS
      stop_count = 0
      pass_count = 0

      data.each do |motion|
        pre_pos = pos
        pre_stop, pre_pass = stop_count, pass_count

        pos += motion

        # did we stop on zero?
        stop_count += 1 if (pos % 100) == 0

        # did we pass zero?
        max_pass = ((pos - pre_pos) / 100).abs
        diff = [pre_pos, pos].sort.map { |p| p / POSITIONS }.reduce(&:-)
        pass_count += diff.abs
        # pass_count -= 1 if pre_pos % 100 == 0 && motion > 0

        # pre_pos -= 1 if (pre_pos % 100) == 0 && motion < 0
        # passes = ((pre_pos / POSITIONS) - (pos / POSITIONS)).abs

        # pass_count += passes if passes > 0

        # pre_stop, pre_pass = stop_count, pass_count
        # pre_pos = pos
        # pos += motion

        # stop_count += 1 if (pos % 100) == 0

        # # how many times did we _pass_ zero

        puts "#{pre_pos} -> #{motion} = #{pos} (#{stop_count}, #{pass_count}) -- (#{stop_count - pre_stop}, #{pass_count - pre_pass})"
      end
      # puts
      stop_count + pass_count
    end

    private
      START_POS = 50
      POSITIONS = 100 # (0-99)

      DIR = { "L" => -1, "R" => 1 }

      def process_input(line)
        data = /([LR])([0-9]+)/.match(line)[1..]
        DIR[data[0]] * data[1].to_i
      end

      def pass_count(pre_pos, cur_pos)
        # pre:95    cur:105  dir:1   pre_stop:0  cur_stop:0  need:(1)
        # pre:105   cur:95   dir:-1  pre_stop:0  cur_stop:0  need:(1)
        # 0, 1, 1, 0, 0    = 1
        # 1, 0, -1, 0, 0   = 1
        #
        # pre:95    cur:100  dir:1   pre_stop:0  cur_stop:1  need:(0)
        # pre:100   cur:95   dir:-1  pre_stop:1  cur_stop:0  need:(0)
        # pre:95    cur:100  dir:1   pre_stop:0  cur_stop:1  need:(0)
        # pre:100   cur:105  dir:1   pre_stop:1  cur_stop:0  need:(0)
        # pre:105   cur:100  dir:-1  pre_stop:0  cur_stop:1  need:(0)
        # 0, 1, 1, 0, 1    = 0
        # 1, 0, -1, 1, 0   = 0
        # 0, 1, 1, 0, 1    = 0
        # 1, 1, -1, 0, 1   = 0
        #
        # pre:95    cur:205  dir:1   pre_stop:0  cur_stop:0  need:(2)
        # pre:205   cur:105  dir:-1  pre_stop:0  cur_stop:0  need:(1)
        # pre:105   cur:200  dir:1   pre_stop:0  cur_stop:1  need:(0)
        # pre:205   cur:100  dir:-1  pre_stop:0  cur_stop:1  need:(1)
        # 0, 2, 1, 0, 0    = 2
        # 2, 1, -1, 0, 0   = 1
        # 1, 2, -1, 0, 1   = 0
        # 2, 1, -1, 0, 1   = 1

        passes = [pre_pos / 100, cur_pos / 100].sort.reduce(:-)
        passes -= 1 if passes > 0
      end
  end
end
