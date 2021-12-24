#!/usr/bin/env ruby

counts = nil
num_lines = 0

open('2021/03/input.txt').each_line do |line|
  bits = line.strip.chars.map &:to_i
  if counts.nil?
    counts = bits
  else
    counts = counts.zip(bits).map { |a, b| a + b }
  end
  num_lines += 1
end

gamma = counts.map { |c| c > (num_lines / 2) ? 1 : 0 }
              .reduce(0) { |sum, num| sum * 2 + num }
epsilon = counts.map { |c| c > (num_lines / 2) ? 0 : 1 }
                .reduce(0) { |sum, num| sum * 2 + num }

puts gamma * epsilon
