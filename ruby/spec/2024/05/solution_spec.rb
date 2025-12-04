# frozen_string_literal: true
require 'spec_helper'

RSpec.describe Year2024::Day05 do
  let(:input) { File.read(File.join(File.dirname(__FILE__), "../../../challenges/2024/05/input.txt")) }
  let(:example_input) {
    <<~EOF
      47|53
      97|13
      97|61
      97|47
      75|29
      61|13
      75|53
      29|13
      97|29
      53|29
      61|53
      97|53
      61|29
      47|13
      75|47
      97|75
      47|61
      75|61
      47|29
      75|13
      53|13

      75,47,61,53,29
      97,61,53,29,13
      75,29,13
      75,97,47,61,53
      61,13,29
      97,13,75,29,47
    EOF
  }

  describe "part 1" do
    it "returns nil for the example input" do
      expect(described_class.part_1(example_input)).to eq(143)
    end

    it "returns nil for my input" do
      expect(described_class.part_1(input)).to eq(3608)
    end
  end

  describe "part 2" do
    it "returns nil for the example input" do
      expect(described_class.part_2(example_input)).to eq(123)
    end

    it "returns nil for my input" do
      expect(described_class.part_2(input)).to eq(4922)
    end
  end
end
