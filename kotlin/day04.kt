package day04
import common.readInputLines

typealias Board = MutableList<MutableList<Int>>

fun parseGame(lines: Iterable<String>): Pair<List<Int>, List<Board>> {
    val linesIt = lines.iterator()
    val nums = linesIt.next().split(",").map {it.toInt()}
    linesIt.next()

    val boards = mutableListOf<Board>()
    var board = mutableListOf<MutableList<Int>>()
    while (linesIt.hasNext()) {
        val line = linesIt.next()
        if (line.isEmpty()) {
            boards.add(board)
            board = mutableListOf<MutableList<Int>>()
        } else {
            board.add(line.trim().split("\\s+".toRegex()).map {it.toInt()}.toMutableList())
        }
    }
    boards.add(board)
    return Pair(nums, boards)
}

fun applyNum(board: Board, num: Int) {
    for (row in board) {
        row.replaceAll {n -> if (n == num) -1 else n}
    }
}

fun isFull(board: Board, pos: Int, is_vert: Boolean): Boolean {
    for (i in 0 until board.size) {
        val n = if (is_vert) board[i][pos] else board[pos][i]
        if (n != -1) {
            return false;
        }
    }
    return true
}

fun hasWinner(board: Board): Boolean {
    return (0 until board.size)
        .any {isFull(board, it, true) || isFull(board, it, false)}
}

fun runSimulation(game: Pair<List<Int>, List<Board>>): List<Int> {
    val (nums, boardsOrig) = game
    val boards = boardsOrig.map {it.toMutableList()}
    var winners = mutableListOf<Int>()
    var scores = mutableListOf<Int>()
    for (n in nums) {
        for ((i, board) in boards.withIndex()) {
            if (winners.contains(i)) {
                continue
            }
            applyNum(board, n)
            if (hasWinner(board)) {
                val s = board.flatten().filter {it != -1}.sum()
                winners.add(i)
                scores.add(s * n)
            }
        }
    }
    return scores
}

fun solution() {
    val scores = runSimulation(parseGame(readInputLines(4)))
    println("Answer 1: ${scores[0]}\nAnswer 2: ${scores.last()}")
}
