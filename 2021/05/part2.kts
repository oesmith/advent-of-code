#!/usr/bin/env -S kotlinc -script
import java.io.File
import kotlin.math.abs

val grid: Array<IntArray> = Array(1000) { IntArray(1000) }

File("2021/05/input.txt").forEachLine {
  val (start, end) = it.split(" -> ")
  val (x1, y1) = start.split(",").map(String::toInt)
  val (x2, y2) = end.split(",").map(String::toInt)

  val numSteps = maxOf(abs(x2 - x1), abs(y2 - y1))
  val xStep = (x2 - x1) / numSteps
  val yStep = (y2 - y1) / numSteps
  for (pos in 0..numSteps) {
    grid[y1 + pos * yStep][x1 + pos * xStep] += 1
  }
}

println(grid.map { it.count { it >= 2 } }.sum())
