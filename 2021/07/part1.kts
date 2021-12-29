#!/usr/bin/env -S kotlinc -script
import java.io.File
import kotlin.math.abs

val input = File("2021/07/input.txt").readText().trim().split(",").map(String::toInt).sorted()

val median =
  if (input.size % 2 == 0) {
    (input[input.size / 2] + input[input.size / 2 - 1]) / 2
  } else {
    input[(input.size + 1) / 2 - 1]
  }

println(input.map { abs(median - it) }.sum())
