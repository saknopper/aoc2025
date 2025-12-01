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

	fmt.Println("part 1: ", rotate(50, lines, true))
	fmt.Println("part 2: ", rotate(50, lines, false))
}

func rotate(startPos int, steps []string, endPositionOnly bool) int {
	atZeroCounter := 0
	currentPos := startPos

	for _, step := range steps {
		amount, err := strconv.Atoi(step[1:])
		if err != nil {
			panic(err)
		}

		switch step[0:1] {
		case "L":
			amount = -amount
		case "R":
			// amount is positive
		default:
			panic("unknown direction")
		}

		if !endPositionOnly {
			atZeroCounter += allPositionsInBetweenMatchingZero(currentPos, amount)
		}

		currentPos = calculateNewPosition(currentPos, amount)

		if endPositionOnly && currentPos == 0 {
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

func allPositionsInBetweenMatchingZero(currentPos int, amount int) int {
	atZeroCounter := 0

	if amount > 0 {
		for i := 1; i <= amount; i++ {
			pos := calculateNewPosition(currentPos, i)
			if pos == 0 {
				atZeroCounter++
			}
		}
	}
	if amount < 0 {
		for i := -1; i >= amount; i-- {
			pos := calculateNewPosition(currentPos, i)
			if pos == 0 {
				atZeroCounter++
			}
		}
	}

	return atZeroCounter
}
