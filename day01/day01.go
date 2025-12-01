package main

import (
	"fmt"
	"strconv"

	"github.com/sknopper/aoc2025/utils"
)

func main() {
	lines, err := utils.ReadLines("day01/input.txt")
	if err != nil {
		panic(err)
	}

	fmt.Println("part 1: ", rotate(50, lines))
}

func rotate(startPos int, steps []string) int {
	atZeroCounter := 0
	currentPos := startPos

	for _, step := range steps {
		amount, err := strconv.Atoi(step[1:])
		if err != nil {
			panic(err)
		}

		switch step[0:1] {
		case "L":
			currentPos = calculateNewPosition(currentPos, -amount)
		case "R":
			currentPos = calculateNewPosition(currentPos, amount)
		default:
			panic("unknown direction")
		}

		fmt.Printf("moved %s%d to %d\n", step[0:1], amount, currentPos)

		if currentPos == 0 {
			atZeroCounter++
		}
	}

	return atZeroCounter
}

func calculateNewPosition(currentPos int, amount int) int {
	newPosition := (currentPos + amount) % 100
	if newPosition < 0 {
		return 100 + newPosition
	}

	return newPosition
}
