package day01
import common.readInputLines

fun solution() {
    val nums = readInputLines(1).map {it.toInt()}

    fun countIncr(nums: Iterable<Int>) = nums.windowed(2).filter {(a, b) -> a < b}.count()

    val res1 = countIncr(nums)
    val res2 = countIncr(nums.windowed(3).map {it.sum()})

    println("Answer 1: $res1\nAnswer 2: $res2")
}
