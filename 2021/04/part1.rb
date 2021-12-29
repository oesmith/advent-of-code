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

def is_winner(rows)
  rows.any? { |cells| cells.all?(&:nil?) } or
    (0...5).any? { |i| rows.all? { |cells| cells[i].nil? } }
end

def find_winner()
  $all_boards.each do |rows|
    if is_winner(rows)
      return rows.map { |cells| cells.map(&:to_i).sum }.sum
    end
  end
  nil
end

numbers.each do |number|
  mark_numbers(number)
  winner = find_winner()
  if winner
    puts winner * number
    break
  end
end
