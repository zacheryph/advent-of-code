# frozen_string_literal: true

require "pry"

class Solution
  def self.part_1(*input)
    new(*input).part_1
  end

  def self.part_2(*input)
    new(*input).part_2
  end

  def initialize(input)
    @input = input
  end

  # if input is?
  #   singleline? result of process_input
  #   multiline? result of process_dataset
  def data
    @data ||=
      input
        .lines(chomp: true)
        .map { process_input(_1) }
        .then { _1.one? ? _1.first : process_dataset(_1) }
  end

  private

  attr_reader :input

  def process_input(line)
    line
  end

  def process_dataset(set)
    set
  end
end
