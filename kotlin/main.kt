import kotlin.system.measureNanoTime

import day01.solution as solution01
import day02.solution as solution02
import day03.solution as solution03
import day04.solution as solution04
import day05.solution as solution05

fun main() {
    val solutions = arrayOf(
        ::solution01,
        ::solution02,
        ::solution03,
        ::solution04,
        ::solution05,
    )

    var totalProblems = 0
    var totalElapsed: Long = 0
    var timings = mutableListOf<Long>()
    for ((idx, s) in solutions.withIndex()) {
        println("--- Day%02d ---".format(idx + 1))
        if (s == null) {
            println("<TODO>")
        } else {
            val t = measureNanoTime {
                s.invoke()
            } / 1000
            println("Elapsed: ${t}µs")
            totalProblems += 1
            totalElapsed += t
            timings.add(t)
        }
    }

    println("Total problems: %d, elapsed: %.4fs".format(totalProblems, totalElapsed / 1000000.0))
    println("Problem timings (µs): ${timings}")
}
