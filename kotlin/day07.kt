package day07
import kotlin.math.*
import common.readInputLines


fun median(nums: List<Double>): Double {
    var nc = nums.sorted()
    val mid = nc.size / 2
    if (nc.size % 2 == 1) {
        return nc[mid]
    } else {
        return (nc[mid - 1] + nc[mid]) / 2.0
    }
}

fun part1(nums: List<Double>): Int {
    val minx = round(median(nums)).toInt()
    return nums.map {abs(it - minx)}.sum().toInt()
}

fun minGradient(p0: Double, f: (Double) -> Double, num_it: Int): Double {
    val EPS = 0.001
    var p = p0
    for (i in 0 until num_it) {
        val pr = f(p + EPS)
        val pl = f(p - EPS)
        val dp = (pr - pl) / (2.0 * EPS)
        p = p - dp * EPS
    }
    return p
}

fun part2(nums: List<Double>): Int {
    val f = {x: Double ->
        var res = 0.0
        for (n in nums) {
            val d = abs(x - n)
            res += d * (d + 1.0) / 2.0
        }
        res
    };
    val minx = minGradient(median(nums), f, 1000)
    return f(round(minx)).toInt()
}

fun solution() {
    val line = readInputLines(7).first()
    val nums = line.split(",").map {it.toDouble()}
    println("Answer 1: ${part1(nums)}\nAnswer 2: ${part2(nums)}")
}
