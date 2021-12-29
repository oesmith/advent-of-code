#!/usr/bin/env -S kotlinc -script
import java.io.File
import kotlin.math.abs

val input = File("2021/07/input.txt").readText().trim().split(",").map(String::toInt).sorted()

fun moveCost(dist: Int): Int = dist * (dist + 1) / 2
fun totalCost(centre: Int): Int = input.map { moveCost(abs(centre - it)) }.sum()

val bestPos = (0..input.maxOrNull()!!).minByOrNull(::totalCost)!!
println(totalCost(bestPos))
