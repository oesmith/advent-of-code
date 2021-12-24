#!/usr/bin/env ruby

count = 0
prev2 = nil
prev1 = nil
last_depth = nil

open('2021/01/input.txt').each_line do |line|
  depth = nil
  current = line.to_i
  if !prev2.nil? && !prev1.nil?
    depth = prev2 + prev1 + current
  end
  prev2 = prev1
  prev1 = current
  if !last_depth.nil? && depth > last_depth
    count += 1
  end
  last_depth = depth
end

puts count
