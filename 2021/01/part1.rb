#!/usr/bin/env ruby

count = 0
last_depth = nil

open('2021/01/input.txt').each_line do |line|
  depth = line.to_i
  if !last_depth.nil? && depth > last_depth
    count += 1
  end
  last_depth = depth
end

puts count
