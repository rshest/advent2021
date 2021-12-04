package common

import java.io.File
import java.io.BufferedReader

fun readInputLines(dayIdx: Int): Iterable<String> {
    val fileName = "../data/%02d.txt".format(dayIdx)
    val reader: BufferedReader = File(fileName).bufferedReader()
    return Iterable { reader.lineSequence().iterator() }
}

fun readInputLines(dayIdx: Int, testIdx: Int): Iterable<String> {
    val fileName = "../data/%02dt%d.txt".format(dayIdx, testIdx)
    val reader: BufferedReader = File(fileName).bufferedReader()
    return Iterable { reader.lineSequence().iterator() }
}
