#!/usr/bin/env ruby
pos = 0
depth = 0

open('2021/02/input.txt').each_line do |line|
  direction, count = line.split ' '
  case direction
  when 'forward'
    pos += count.to_i
  when 'down'
    depth += count.to_i
  when 'up'
    depth -= count.to_i
  end
end

puts pos * depth
