# frozen_string_literal: true
module Year2022
  class Day09 < Solution
    def part_1
      snake = Snake.new(1)
      motions.each { |m| snake.move(*m) }
      snake.visited.length
    end

    def part_2
      snake = Snake.new(9)
      motions.each { |m| snake.move(*m) }
      snake.visited.length
    end

    private

    class Snake
      attr_accessor :head, :tail, :visited

      MOVEMENT = {
        "U" => [ 1,  0],
        "D" => [-1,  0],
        "L" => [ 0, -1],
        "R" => [ 0,  1],
      }

      def initialize(length = 1)
        @head = [0, 0]
        @tail = length.times.map { [0, 0] }
        @visited = Set.new([[0, 0]])
      end

      def touching?(head, tail)
        head
          .zip(tail)
          .map { |pos| pos.reduce(:-).abs }
          .all? { |n| n <= 1 }
      end

      def move(dir, movements)
        movements.to_i.times do
          @head = head.zip(MOVEMENT[dir]).map(&:sum)
          last = @head

          @tail = tail.map do |t|
            next last = t if touching?(last, t)
            t = t.dup

            delta_0 = last.first - t.first
            delta_1 = last.last - t.last

            t[0] += delta_0 / delta_0.abs if delta_0.abs > 0
            t[1] += delta_1 / delta_1.abs if delta_1.abs > 0

            last = t
          end
          visited.add(tail.last)
        end
      end
    end

    def motions
      input.each_line.map { |l| l.split }
    end
  end
end
