# frozen_string_literal: true
module Year2022
  class Day07 < Solution
    def part_1
      dirs = []

      filesystem.walk do |entry|
        next if entry.is_a?(File)
        dirs << entry if entry.size <= 100000
      end

      dirs.map(&:size).sum
    end

    def part_2
      need_to_free = 30000000 - (70000000 - filesystem.size)
      dir = filesystem

      filesystem.walk do |entry|
        next if entry.is_a?(File)
        next if entry.size < need_to_free
        dir = entry if entry.size < dir.size
      end

      dir.size
    end

    private

    File = Struct.new(:path, :size, keyword_init: true) do
      def walk(&block)
        # nop
        # block.call(self)
      end
    end

    Dir = Struct.new(:path, keyword_init: true) do
      def add(entry)
        elements[entry.path] ||= entry
      end

      def size
        elements.values.reduce(0) { |s, v| s + v.size }
      end

      def elements
        @elements ||= {}
      end

      def walk(&block)
        elements.values.each { |element| element.walk(&block) }
        block.call(self)
      end
    end

    def filesystem
      @filesystem ||= populate_root
    end

    def populate_root
      fs = Dir.new(path: nil)
      ptr = [fs]
      reader = input.each_line

      begin
        while line = reader.next.split
          case line
          in ["$", "cd", ".."]
            ptr.pop
          in ["$", "cd", path]
            dir = Dir.new(path:)
            ptr.push(ptr.last.add(dir))
          in ["$", "ls"]
            # nop
          in ["dir", path]
            ptr.last.add(Dir.new(path:))
          in [size, path]
            ptr.last.add(File.new(path:, size: size.to_i))
          end
        end
      rescue StopIteration
      end
      fs
    end
  end
end
