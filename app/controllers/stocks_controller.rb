require 'csv'
class StocksController < ApplicationController
  def index
  end

  def import
    file = params[:file]
    t0 = Time.now
    data = Hash.new { |h, k| h[k] = [] }
    CSV.foreach(file.path, headers: true) do |row|
      symbol = row.to_hash["symbol"]
      data[symbol] << row.to_hash
    end

    data.keys.each do |symbol|
      data[symbol] = data[symbol].sort do |a,b|
        b["close"].to_f <=> a["close"].to_f
      end
    end

    t1 = Time.now
    ellapsed_time = ((t1 - t0) * 1000.0).round(4)
    render json: {time: ellapsed_time, data: data.keys.reduce({}){ |memo, k| memo[k] = data[k][0...10]; memo }}
  end
end