package day06
import common.readInputLines

fun getNumSpawned(n: Long, days: Long, spawned: HashMap<Pair<Long, Long>, Long>): Long {
    if (days <= n) {
        return 0
    }
    val key = Pair(n, days)
    if (spawned.containsKey(key)) {
        return spawned.get(key)!!
    }
    var res = (days - n - 1) / 7 + 1
    for (i in 0 .. res) {
        res += getNumSpawned(8, days - n - i * 7 - 1, spawned)
    }
    spawned.put(key, res)
    return res
}

fun sumSpawned(nums: List<Long>, totalDays: Long): Long {
    val spawned = HashMap<Pair<Long, Long>, Long>()
    return nums.map {getNumSpawned(it, totalDays, spawned)}.sum() + nums.size
}

fun solution() {
    val line = readInputLines(6).first()
    val nums = line.split(",").map {it.toLong()}
    val res1 = sumSpawned(nums, 80)
    val res2 = sumSpawned(nums, 256)
    println("Answer 1: ${res1}\nAnswer 2: ${res2}")
}
