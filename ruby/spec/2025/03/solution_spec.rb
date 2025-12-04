# frozen_string_literal: true
require 'spec_helper'

RSpec.describe Year2025::Day03 do
  let(:input) { File.read(File.join(File.dirname(__FILE__), "../../../challenges/2025/03/input.txt")) }
  let(:example_input) { File.read(File.join(File.dirname(__FILE__), "../../../challenges/2025/03/example.txt")) }

  describe "part 1" do
    fit "returns nil for the example input" do
      expect(described_class.part_1(example_input)).to eq(357)
    end

    it "returns nil for my input" do
      expect(described_class.part_1(input)).to eq(17193)
    end
  end

  describe "part 2" do
    it "returns nil for the example input" do
      expect(described_class.part_2(example_input)).to eq(3121910778619)
    end

    it "returns nil for my input" do
      expect(described_class.part_2(input)).to eq(171297349921310)
    end
  end
end
