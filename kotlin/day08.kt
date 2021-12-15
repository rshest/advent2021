package day08
import common.readInputLines

typealias Mapping = MutableList<MutableList<Int>>

val CHARS = "abcdefg"
val DIGITS = arrayOf(
    "abcefg", "cf", "acdeg", "acdfg", "bcdf",
    "abdfg", "abdefg", "acf", "abcdefg", "abcdfg")
val NUM_SEGMENTS = 7

fun normalizeMapping(mapping: Mapping) {
    var changed = true
    while (changed) {
        changed = false;
        for (i in 0 until mapping.size) {
            val c = mapping[i]
            if (c.sum() == 1) {
                val p = c.indexOf(1)
                for (j in 0 until NUM_SEGMENTS) {
                    if (j != i && mapping[j][p] == 1) {
                        mapping[j][p] = 0
                        changed = true
                    }
                }
            }
        }
    }
}

fun isValidMapping(mapping: Mapping): Boolean {
    for (i in 0 until NUM_SEGMENTS) {
        if (mapping.map {it[i]}.sum() != 1) {
            return false
        }
    }
    return mapping.map {it.sum()}.all {it == 1}
}

fun getMapping(mapping: Mapping, combs: List<String>, pos: Int): Mapping? {
    if (pos == combs.size) {
        normalizeMapping(mapping)
        if (isValidMapping(mapping)) {
            return mapping
        } else {
            return null
        }
    }

    val comb = combs[pos]
    for (digit in DIGITS) {
        if (digit.length != comb.length) {
            continue
        }
        val mapping1 = mapping.toMutableList()
        for (d in CHARS) {
            if (!digit.contains(d)) {
                for (c in comb) {
                    mapping1[c.code - 'a'.code][d.code - 'a'.code] = 0
                }
            }
        }
        val resMapping = getMapping(mapping1, combs, pos + 1)
        if (resMapping != null) {
            return resMapping
        }
    }
    return null
}

fun decodeDigit(mapping: Mapping, combs: List<String>): Int {
    var res = 0
    for (comb in combs) {
        var scomb = comb.toMutableList()
        for (i in 0 until comb.length) {
            scomb[i] = (mapping[scomb[i].code - 'a'.code].indexOf(1) + 'a'.code).toChar()
        }
        scomb.sort();
        //let s = str::from_utf8(&scomb).unwrap();
        //let digit = DIGITS.iter().position(|&e| e == s).unwrap();
        //res = res * 10 + digit
    }
    return res
}

fun decodeSum(inp: List<List<List<String>>>): Int {
    var res = 0
    for ((combs, toDecode) in inp) {
        var mapping: Mapping = MutableList(NUM_SEGMENTS) { MutableList(NUM_SEGMENTS) {1} }
        val combsSorted = combs.sortedBy { it.length }
        val resMapping = getMapping(mapping, combsSorted, 0)
        res += decodeDigit(resMapping!!, toDecode)
    }
    return res
}

fun solution() {
    val inp = readInputLines(8, 1).map {it.split("|").map {it.trim().split(" ")}}
    val res1 = inp
        .map {line -> line[1].filter {listOf(2, 3, 4, 7).contains(it.length)}.count()}
        .sum()
    val res2 = decodeSum(inp)
    println("Answer 1: ${res1}\nAnswer 2: ${res2}")
}
