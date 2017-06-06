require 'helix_runtime/build_task'

HelixRuntime::BuildTask.new("top_stocks")

task :default => :build
