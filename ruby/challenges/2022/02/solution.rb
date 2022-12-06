# frozen_string_literal: true
module Year2022
  class Day02 < Solution
    def part_1
      pick_map = { "X" => :rock, "Y" => :paper, "Z" => :scissor }

      games
        .map { |elf, me| game_score(elf, pick_map[me]) }
        .sum
    end

    def part_2
      pick_map = { "X" => WINNER, "Y" => DRAW, "Z" => LOSER }

      games
        .map { |elf, me| [elf, pick_map[me][elf]] }
        .map { |elf, me| game_score(elf, me) }
        .sum
    end

    private

    SHAPE_MAP = { "A" => :rock, "B" => :paper, "C" => :scissor }
    SHAPE_SCORE = { rock: 1, paper: 2, scissor: 3 }

    WINNER = {
      rock:    :scissor,
      paper:   :rock,
      scissor: :paper,
    }.freeze

    LOSER = WINNER.invert.freeze
    DRAW = WINNER.keys.to_h { |k| [k, k] }.freeze

    def games
      input
        .split("\n")
        .map(&:split)
        .map { |elf, us| [SHAPE_MAP[elf], us] }
    end

    def game_score(elf, me)
      case elf
      when WINNER[me]
        6
      when LOSER[me]
        0
      else # draw
        3
      end + SHAPE_SCORE[me]
    end

    def games
      input
        .split("\n")
        .map { |g| g.split.map { SHAPE_MAP[_1] || _1 } }
    end
  end
end
