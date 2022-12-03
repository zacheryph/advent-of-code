# frozen_string_literal: true
module Year2022
  class Day02 < Solution
    def part_1
      games.map { |elf, me| SHAPE_SCORE[me] + GAME_SCORE[[elf, me]] }.sum
    end

    def part_2
      games.map do |elf, me|
        me = part_2_pick(elf, me)
        SHAPE_SCORE[me] + GAME_SCORE[[elf, me]]
      end.sum
    end

    private

    SHAPE_MAP = {
      "A" => :rock,
      "B" => :paper,
      "C" => :scissor,
      "X" => :rock,
      "Y" => :paper,
      "Z" => :scissor,
    }

    SHAPE_SCORE = {
      rock:    1,
      paper:   2,
      scissor: 3,
    }

    GAME_SCORE = {
      [:rock, :rock] => 3,
      [:rock, :paper] => 6,
      [:rock, :scissor] => 0,
      [:paper, :rock] => 0,
      [:paper, :paper] => 3,
      [:paper, :scissor] => 6,
      [:scissor, :rock] => 6,
      [:scissor, :paper] => 0,
      [:scissor, :scissor] => 3,
    }

    # this is ugly. rock=lose, paper=draw, scissor=win
    def part_2_pick(elf, me)
      case me
      when :rock
        SHAPE_SCORE.keys[SHAPE_SCORE.keys.index(elf) - 1]
      when :paper
        elf
      when :scissor
        SHAPE_SCORE.keys[(SHAPE_SCORE.keys.index(elf) + 1) % 3]
      end
    end

    def score(game)
      me = game.last
      SHAPE_SCORE[me] + GAME_SCORE[game]
    end

    def games
      input
        .split("\n")
        .map { |g| g.split.map { SHAPE_MAP[_1] } }
    end
  end
end
