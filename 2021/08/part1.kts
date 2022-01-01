#!/usr/bin/env -S kotlinc -script
import java.io.File

var count = 0
File("2021/08/input.txt").forEachLine {
  val (_, after) = it.split(" | ").map { it.split(" ") }
  for (digit in after) {
    if (digit.length == 2 ||
        digit.length == 3 ||
        digit.length == 4 ||
        digit.length == 7) {
      count += 1
    }
  }
}
println(count)
