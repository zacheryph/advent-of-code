# frozen_string_literal: true
require 'spec_helper'

RSpec.describe Year2024::Day07 do
  let(:input) { File.read(File.join(File.dirname(__FILE__), "../../../challenges/2024/07/input.txt")) }
  let(:example_input) {
    <<~EOF
      190: 10 19
      3267: 81 40 27
      83: 17 5
      156: 15 6
      7290: 6 8 6 15
      161011: 16 10 13
      192: 17 8 14
      21037: 9 7 18 13
      292: 11 6 16 20
    EOF
  }

  describe "part 1" do
    it "returns nil for the example input" do
      expect(described_class.part_1(example_input)).to eq(3749)
    end

    it "returns nil for my input" do
      expect(described_class.part_1(input)).to eq(1582598718861)
    end
  end

  describe "part 2" do
    it "returns nil for the example input" do
      expect(described_class.part_2(example_input)).to eq(nil)
    end

    it "returns nil for my input" do
      expect(described_class.part_2(input)).to eq(nil)
    end
  end
end
