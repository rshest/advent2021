package day05
import common.readInputLines

typealias Point = Pair<Int, Int>
typealias Line = Pair<Point, Point>

fun parsePt(pt: String): Point {
    val xy = pt.split(",").map {it.toInt()}
    return Pair(xy[0], xy[1])
}

fun parseLine(line: String): Line {
    val ab = line.split(" -> ").map(::parsePt)
    return Pair(ab[0], ab[1])
}

fun sgn(x: Int): Int {
    if (x == 0) return 0
    if (x < 0) return -1 else return 1
}

fun stroke(a: Point, b: Point, ptmap: HashMap<Point, Int>, skipDiagonals: Boolean) {
    val (x1, y1) = a
    val (x2, y2) = b
    val dx = sgn(x2 - x1)
    val dy = sgn(y2 - y1)
    if (skipDiagonals && dx != 0 && dy != 0) {
        return;
    }
    var cx = x1
    var cy = y1
    while (true) {
        val cpt = Pair(cx, cy)
        ptmap[cpt] = ptmap.getOrDefault(cpt, 0) + 1
        if (cx == x2 && cy == y2) {
            break;
        }
        cx += dx
        cy += dy
    }
}

fun findNumOverlaps(lines: List<Line>, skipDiagonals: Boolean): Int {
    var ptmap = HashMap<Point, Int>()
    for ((a, b) in lines) {
        stroke(a, b, ptmap, skipDiagonals);
    }
    return ptmap.map {(_, cnt) -> if (cnt >= 2) 1 else 0}.sum()
}

fun solution() {
    val lines = readInputLines(5).map(::parseLine)
    val res1 = findNumOverlaps(lines, true)
    val res2 = findNumOverlaps(lines, false)
    println("Answer 1: ${res1}\nAnswer 2: ${res2}")
}
