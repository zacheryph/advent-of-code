# frozen_string_literal: true
module Year2023
  class Day07 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data.sort
        .reverse
        .each_with_index
        .map { |hand, idx| hand.bid * (idx + 1) }
        .sum
    end

    def part_2
      data
        .each(&:wild!)
        .sort
        .reverse
        .each_with_index
        .map { |hand, idx| hand.bid * (idx + 1) }
        .sum
    end

    private
      # Processes each line of the input file and stores the result in the dataset
      def process_input(line)
        Hand.new(*line.split)
      end

      # Processes the dataset as a whole
      # def process_dataset(set)
      #   set
      # end

      class Hand
        CARD_STRENGTH = "23456789TJQKA"
        WILD_STRENGTH = "J23456789TQKA"

        STRENGTH_PAIRS = [
          {5 => 1},         # five of a kind
          {4 => 1},         # four of a kind
          {3 => 1, 2 => 1}, # fullhouse
          {3 => 1},         # three of a kind
          {2 => 2},         # two pair
          {2 => 1},         # pair
        ].freeze

        attr_reader :bid, :cards, :strength, :wild

        def initialize(cards, bid, wild: false)
          @cards = cards.split("")
          @bid = bid.to_i
        end

        def wild!
          @wild = true
        end

        def card_strengths
          cards.map { |c| (wild ? WILD_STRENGTH : CARD_STRENGTH).index(c) }
        end

        def <=>(other)
          case (str = self.strength <=> other.strength)
          when -1, 1
            str
          else
            other.card_strengths
              .zip(card_strengths)
              .map { |pair| pair[0] <=> pair[1] }
              .find { |n| n != 0 }
          end
        end

        def counts
          @counts ||= cards.tally.values.tally
        end

        def wild_counts
          (cards - ["J"]).tally.values.tally
        end

        def strength
          str_counts = wild ? wild_counts : counts

          str = STRENGTH_PAIRS.find do |pairs|
            pairs.all? { |c, t| str_counts[c] == t }
          end || {1 => 1}

          wilds = cards.tally["J"]
          return 0 if wilds == 5
          if wild && wilds
            str = str.dup
            top_pair = str.keys.max
            str[top_pair + wilds] = 1
            str[top_pair] -= 1
            str.reject! { |k, v| v == 0 }
          end

          STRENGTH_PAIRS.index(str) || STRENGTH_PAIRS.length
        end
      end
  end
end
