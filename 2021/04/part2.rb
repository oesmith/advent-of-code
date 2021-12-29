#!/usr/bin/env ruby

input = open('2021/04/input.txt')

numbers = input.readline.strip.split(',').map &:to_i

$all_boards = []

until input.eof?
  input.readline  # Blank line.

  board = []
  5.times do
    board << input.readline.strip.split(/\s+/).map(&:to_i)
  end
  $all_boards << board
end

def mark_numbers(number)
  $all_boards.map! do |rows|
    rows.map! do |cells|
      cells.map! { |cell| cell == number ? nil : cell }
    end
  end
end

def is_winner?(rows)
  rows.any? { |cells| cells.all?(&:nil?) } ||
    (0...5).any? { |i| rows.all? { |cells| cells[i].nil? } }
end

def sum_winner(rows)
  rows.map { |cells| cells.map(&:to_i).sum }.sum
end

numbers.each do |number|
  mark_numbers(number)
  winners, $all_boards = $all_boards.partition { |r| is_winner?(r) }
  if $all_boards.empty?
    puts sum_winner(winners.last) * number
    break
  end
end
