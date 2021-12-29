#!/usr/bin/env -S kotlinc -script
import java.io.File

var fish = LongArray(9)

for (f in File("2021/06/input.txt").readText().trim().split(",").map(String::toInt)) {
  fish[f] = fish[f] + 1
}

val iterations = 80
for (i in 0 until iterations) {
  val newFish = fish[0]
  for (j in 0..7) {
    fish[j] = fish[j + 1]
  }
  fish[6] += newFish
  fish[8] = newFish
}

println(fish.sum())
