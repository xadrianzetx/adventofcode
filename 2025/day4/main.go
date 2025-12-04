package main

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed input
var data string

type Position struct {
	x int
	y int
}

func MakePosition(x int, y int) Position {
	return Position{x, y}
}

func MakeGrid(data string) map[Position]bool {
	grid := make(map[Position]bool)
	rowNum := 0

	for row := range strings.Lines(data) {
		for colNum, char := range row {
			if char == '@' {
				pos := MakePosition(colNum, rowNum)
				grid[pos] = true
			}
		}

		rowNum += 1
	}

	return grid
}

func GetAccessibleRolls(grid map[Position]bool) []Position {
	var accessible []Position

	for roll := range grid {
		if IsRollAccessible(roll, grid) {
			accessible = append(accessible, roll)
		}
	}

	return accessible
}

func IsRollAccessible(roll Position, grid map[Position]bool) bool {
	adjacent := []int{-1, 0, 1}
	numNeighbors := 0

	for _, col := range adjacent {
		for _, row := range adjacent {
			maybeNeighborRoll := MakePosition(roll.x+col, roll.y+row)
			if maybeNeighborRoll != roll {
				_, ok := grid[maybeNeighborRoll]
				if ok {
					numNeighbors += 1
				}
			}
		}
	}

	return numNeighbors < 4
}

func RemoveAccessibleRolls(grid map[Position]bool, accessible []Position) {
	for _, roll := range accessible {
		delete(grid, roll)
	}
}

func main() {
	grid := MakeGrid(data)

	totalRollsRemoved := 0
	accessible := GetAccessibleRolls(grid)
	fmt.Println("Part 1:", len(accessible))

	for len(accessible) > 0 {
		accessible = GetAccessibleRolls(grid)
		RemoveAccessibleRolls(grid, accessible)
		totalRollsRemoved += len(accessible)
	}

	fmt.Println("Part 2:", totalRollsRemoved)
}
