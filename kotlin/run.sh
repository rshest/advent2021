#/usr/bin/bash

kotlinc *.kt -include-runtime -d build/aoc2021.jar && java -jar build/aoc2021.jar -server
