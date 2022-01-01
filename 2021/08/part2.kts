#!/usr/bin/env -S kotlinc -script
import java.io.File

val allSegments = setOf('a', 'b', 'c', 'd', 'e', 'f', 'g')

fun invertDigit(digit: Set<Char>): Set<Char> = allSegments.subtract(digit)

var total = 0
File("2021/08/input.txt").forEachLine {
  val (before, after) = it.split(" | ").map { it.split(" ").map { it.toSet() } }

  val unparsedDigits = before.toMutableList()
  val parsedDigits = Array<Set<Char>?>(10) { null }

  fun findDigit(index: Int, fn: (digit: Set<Char>) -> Boolean) {
    val i = unparsedDigits.indexOfFirst(fn)
    check(i != -1) { "No matches for #${index}" }
    parsedDigits[index] = unparsedDigits.elementAt(i)
    unparsedDigits.removeAt(i)
  }

  // First pass -- obvious digits.
  findDigit(1) { it.size == 2 }
  findDigit(7) { it.size == 3 }
  findDigit(4) { it.size == 4 }
  findDigit(8) { it.size == 7 }
  // Infer the remaining digits based on the segments in previous digits.
  // Ordering is important for these!
  findDigit(9) { it.size == 6 && it.containsAll(parsedDigits[4]!!) }
  findDigit(0) { it.size == 6 && it.containsAll(parsedDigits[1]!!) }
  findDigit(6) { it.size == 6 } // the only remaining 6-segment digit.
  findDigit(3) { it.size == 5 && it.containsAll(parsedDigits[1]!!) }
  findDigit(2) { it.size == 5 && it.containsAll(invertDigit(parsedDigits[9]!!)) }
  findDigit(5) { it.size == 5 } // the only remaining 5-segment digit.

  var output = 0
  for (digit in after) {
    val i = parsedDigits.indexOf(digit)
    check(i != -1) { "No matches for ${digit.joinToString("")}"}
    output = output * 10 + i
  }
  total += output
}

println(total)
