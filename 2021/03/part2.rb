#!/usr/bin/env ruby

nums = open('2021/03/input.txt').readlines.map { |l| l.strip.chars.map &:to_i }

num_bits = nums.first.size  # Assume all numbers have the same bit width.

oxygen = nums
(0...num_bits).each do |i|
  count = oxygen.reduce(0) { |sum, bits| sum + bits[i] }
  common = (count >= (oxygen.size / 2.0)) ? 1 : 0
  oxygen = oxygen.reject { |x| x[i] != common }
  if oxygen.size == 1
    oxygen = oxygen.first.reduce(0) { |sum, num| sum * 2 + num }
    break
  end
end

co2 = nums
(0...num_bits).each do |i|
  count = co2.reduce(0) { |sum, bits| sum + bits[i] }
  common = (count < (co2.size / 2.0)) ? 1 : 0
  co2 = co2.reject { |x| x[i] != common }
  if co2.size == 1
    co2 = co2.first.reduce(0) { |sum, num| sum * 2 + num }
    break
  end
end

puts oxygen * co2
