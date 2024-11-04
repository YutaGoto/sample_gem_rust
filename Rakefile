# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("sample_gem_rust.gemspec")

RbSys::ExtensionTask.new("sample_gem_rust", GEMSPEC) do |ext|
  ext.lib_dir = "lib/sample_gem_rust"
end

task default: %i[compile spec]
