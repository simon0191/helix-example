require "helix_runtime"

begin
  require "top_stocks/native"
rescue LoadError
  warn "Unable to load top_stocks/native. Please run `rake build`"
end
