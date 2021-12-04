package day03
import common.readInputLines

fun getMostCommon(nums: Iterable<List<Int>>, pos: Int): Int {
    var n0 = 0;
    var n1 = 0;
    for (num in nums) {
        n0 += 1 - num[pos];
        n1 += num[pos];
    }
    if (n1 >= n0) {
        return 1
    } else {
        return 0
    }
}

fun digitsToDecimal(digits: Iterable<Int>): Int {
    return digits.fold(0) {v, d -> v * 2 + d}
}


fun part1(nums: List<List<Int>>): Int {
    var lc = mutableListOf<Int>()
    var mc = mutableListOf<Int>()
    for (i in 0 until nums[0].size) {
        val m = getMostCommon(nums, i)
        mc.add(m)
        lc.add(1 - m)
    }
    return digitsToDecimal(mc) * digitsToDecimal(lc)
}

fun part2(nums: List<List<Int>>): Int {
    var mnums = (0 until nums.size).toList()
    var lnums = (0 until nums.size).toList()
    for (i in 0 until nums[0].size) {
        if (mnums.size > 1) {
            val m = getMostCommon(mnums.map {nums[it]}, i)
            mnums = mnums.filter {nums[it][i] == m }
        }
        if (lnums.size > 1) {
            val m = getMostCommon(lnums.map {nums[it]}, i)
            lnums = lnums.filter {nums[it][i] == 1 - m }
        }
    }
    return digitsToDecimal(nums[mnums[0]]) * digitsToDecimal(nums[lnums[0]])
}

fun solution() {
    val nums = readInputLines(3).map {it.toCharArray().map {Character.getNumericValue(it)}}
    println("Answer 1: ${part1(nums)}\nAnswer 2: ${part2(nums)}")
}
