# frozen_string_literal: true
require 'spec_helper'

RSpec.describe Year2015::Day01 do
  let(:input) { File.read(File.join(File.dirname(__FILE__), "../../../challenges/2015/01/input.txt")) }
  let(:example_input) {
    <<~EOF
      ))(((((
    EOF
  }

  describe "part 1" do
    it "returns nil for the example input" do
      expect(described_class.part_1(example_input)).to eq(3)
    end

    it "returns nil for my input" do
      expect(described_class.part_1(input)).to eq(280)
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
