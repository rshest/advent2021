import time

import day01
import day02
import day03
import day04
import day05
import day06
import day07
import day08
import day09
import day10
import day11
import day12
import day13
import day14
import day15
import day16
import day17
import day18
import day19
import day20
import day21

SOLUTIONS = [
    day01.solution,
    day02.solution,
    day03.solution,
    day04.solution,
    day05.solution,
    day06.solution,
    day07.solution,
    day08.solution,
    day09.solution,
    day10.solution,
    day11.solution,
    day12.solution,
    day13.solution,
    day14.solution,
    day15.solution,
    day16.solution,
    day17.solution,
    day18.solution,
    day19.solution,
    day20.solution,
    day21.solution,
]


def main():
    total_problems = 0
    total_elapsed = 0
    timings = []
    for (i, solution) in enumerate(SOLUTIONS):
        problem_id = i + 1
        print(f"--- Day{problem_id:02} ---")
        if solution == None:
            print("<TODO>")
        else:
            start = time.time()
            solution()
            end = time.time()
            elapsed = end - start
            total_problems += 1
            total_elapsed += elapsed
            timings.append(int(round(elapsed * 1000000.0)))
            print(f"Elapsed: {timings[i]}µs")

    print(f"Total problems: {total_problems}, elapsed: {total_elapsed:.4f}s")
    print(f"Problem timings (µs): {timings}")


if __name__ == "__main__":
    main()
