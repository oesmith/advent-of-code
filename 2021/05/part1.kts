#!/usr/bin/env -S kotlinc -script
import java.io.File

val grid: Array<IntArray> = Array(1000) { IntArray(1000) }

File("2021/05/input.txt").forEachLine {
  val (start, end) = it.split(" -> ")
  val (x1, y1) = start.split(",").map(String::toInt)
  val (x2, y2) = end.split(",").map(String::toInt)

  if (x1 == x2) {
    // Vertical line.
    for (y in minOf(y1, y2)..maxOf(y1, y2)) {
      grid[y][x1] += 1
    }
  } else if (y1 == y2) {
    // Horizontal line.
    for (x in minOf(x1, x2)..maxOf(x1, x2)) {
      grid[y1][x] += 1
    }
  }
}

println(grid.map { it.count { it >= 2 } }.sum())
