# frozen_string_literal: true
require 'spec_helper'

RSpec.describe Year2025::Day06 do
  let(:input) { File.read(File.join(File.dirname(__FILE__), "../../../challenges/2025/06/input.txt")) }
  let(:example_input) { File.read(File.join(File.dirname(__FILE__), "../../../challenges/2025/06/example.txt")) }

  describe "part 1" do
    it "returns nil for the example input" do
      expect(described_class.part_1(example_input)).to eq(4277556)
    end

    it "returns nil for my input" do
      expect(described_class.part_1(input)).to eq(5524274308182)
    end
  end

  describe "part 2" do
    it "returns nil for the example input" do
      expect(described_class.part_2(example_input)).to eq(3263827)
    end

    it "returns nil for my input" do
      expect(described_class.part_2(input)).to eq(8843673199391)
    end
  end
end
