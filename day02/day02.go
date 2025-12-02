package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"

	"github.com/sknopper/aoc2025/utils"
)

func main() {
	lines, err := utils.ReadLines("day02/input.txt")
	if err != nil {
		panic(err)
	}

	ranges := strings.SplitSeq(lines[0], ",")
	sumA := 0
	sumB := 0
	for r := range ranges {
		parts := strings.Split(r, "-")
		start, _ := strconv.Atoi(parts[0])
		end, _ := strconv.Atoi(parts[1])

		for i := start; i <= end; i++ {
			iStr := strconv.Itoa(i)
			iStrLen := len(iStr)

			stepsToCheck := iStrLen / 2

			equalsSteps := []int{}

			for j := stepsToCheck; j > 0; j-- {
				chunkLength := iStrLen / j
				if j == 1 {
					chunkLength = 1
				}

				if AllChunksOfLengthAreEqual(iStr, chunkLength) {
					equalsSteps = append(equalsSteps, j)
				}
			}

			if len(equalsSteps) > 0 {
				sumB += i
				if slices.Contains(equalsSteps, 2) || (iStrLen == 2 && slices.Contains(equalsSteps, 1)) {
					sumA += i
				}
			}
		}
	}

	fmt.Println("part 1: ", sumA)
	fmt.Println("part 2: ", sumB)
}

func AllChunksOfLengthAreEqual(s string, length int) bool {
	sLen := len(s)
	if sLen == length || sLen%length != 0 {
		return false
	}

	chunkCount := sLen / length
	firstChunk := s[:length]

	for i := 1; i < chunkCount; i++ {
		if s[i*length:(i+1)*length] != firstChunk {
			return false
		}
	}

	return true
}
