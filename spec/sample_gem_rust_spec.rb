# frozen_string_literal: true

RSpec.describe SampleGemRust do
  it "has a version number" do
    expect(SampleGemRust::VERSION).not_to be nil
  end

  it "does something useful" do
    expect(SampleGemRust.hello("world")).to eq("Hello from Rust, world!")
  end
end
