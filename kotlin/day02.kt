package day02
import common.readInputLines

enum class Command {
    forward, up, down
}

fun parseLine(line: String): Pair<Command, Int> {
    val (c, n) = line.split(" ")
    return Pair(Command.valueOf(c), n.toInt())
}

fun step1(pos: Pair<Int, Int>, cmdLine: Pair<Command, Int>): Pair<Int, Int> {
    val (x, depth) = pos
    val (cmd, offs) = cmdLine
    return when (cmd) {
        Command.up -> Pair(x, depth - offs)
        Command.down -> Pair(x, depth + offs)
        Command.forward -> Pair(x + offs, depth)
    }
}

fun step2(pos: Triple<Int, Int, Int>, cmdLine: Pair<Command, Int>): Triple<Int, Int, Int> {
    val (x, depth, aim) = pos
    val (cmd, offs) = cmdLine
    return when (cmd) {
        Command.up -> Triple(x, depth, aim - offs)
        Command.down -> Triple(x, depth, aim + offs)
        Command.forward -> Triple(x + offs, depth + aim * offs, aim)
    }
}

fun solution() {
    val commands = readInputLines(2).map {parseLine(it)}
    val (a1, b1) = commands.fold(Pair(0, 0), ::step1)
    val (a2, b2) = commands.fold(Triple(0, 0, 0), ::step2)
    println("Answer 1: ${a1 * b1}\nAnswer 2: ${a2 * b2}")
}
