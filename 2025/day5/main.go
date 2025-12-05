package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

//go:embed input
var data string

type IngredientRange struct {
	lower int
	upper int
}

func MakeIngredientRanges(raw string) []IngredientRange {
	var ingredientRanges []IngredientRange

	for line := range strings.Lines(raw) {
		leftAndRight := strings.Split(strings.TrimSpace(line), "-")
		lower, _ := strconv.Atoi(leftAndRight[0])
		upper, _ := strconv.Atoi(leftAndRight[1])
		ingredientRanges = append(ingredientRanges, IngredientRange{lower: lower, upper: upper})
	}

	return ingredientRanges
}

func IsFresh(ingredient int, ingredientRanges []IngredientRange) bool {
	for _, ingredientRange := range ingredientRanges {
		if ingredient >= ingredientRange.lower && ingredient <= ingredientRange.upper {
			return true
		}
	}
	return false
}

func FuseRangesUntilAllDisjoint(ingredientRanges []IngredientRange) []IngredientRange {
	var changed bool
	for {
		ingredientRanges, changed = FuseRanges(ingredientRanges)
		if !changed {
			return ingredientRanges
		}
	}
}

func FuseRanges(ingredientRanges []IngredientRange) ([]IngredientRange, bool) {
	fusedIngredientRanges := []IngredientRange{ingredientRanges[0]}

	for _, ingredientRange := range ingredientRanges {
		var newFused []IngredientRange
		wasFused := false
		for _, fusedIngredientRange := range fusedIngredientRanges {
			newRange, fused := MaybeFuse(ingredientRange, fusedIngredientRange)
			if fused {
				newFused = append(newFused, newRange)
				wasFused = true
			} else {
				newFused = append(newFused, fusedIngredientRange)
			}
		}

		if !wasFused {
			newFused = append(newFused, ingredientRange)
		}

		fusedIngredientRanges = newFused
	}

	return fusedIngredientRanges, len(ingredientRanges) != len(fusedIngredientRanges)
}

func MaybeFuse(a IngredientRange, b IngredientRange) (IngredientRange, bool) {
	if a.lower > b.upper || a.upper < b.lower {
		// Disjoint.
		return IngredientRange{}, false

	} else if (a.lower >= b.lower && a.upper <= b.upper) || (b.lower >= a.lower && b.upper <= a.upper) {
		// Contains.
		newRange := IngredientRange{lower: min(a.lower, b.lower), upper: max(a.upper, b.upper)}
		return newRange, true

	} else if a.lower < b.lower && a.upper >= b.lower {
		// Left overlap.
		newRange := IngredientRange{lower: a.lower, upper: b.upper}
		return newRange, true

	} else {
		// Right overlap.
		newRange := IngredientRange{lower: b.lower, upper: a.upper}
		return newRange, true
	}
}

func main() {
	rangesAndNumbers := strings.Split(data, "\n\n")
	ingredientRanges, ingredients := MakeIngredientRanges(rangesAndNumbers[0]), rangesAndNumbers[1]

	freshIngredientCount := 0
	for ingredientId := range strings.Lines(ingredients) {
		ingredient, _ := strconv.Atoi(strings.TrimSpace(ingredientId))
		if IsFresh(ingredient, ingredientRanges) {
			freshIngredientCount += 1
		}
	}
	fmt.Println("Part 1:", freshIngredientCount)

	totalPossibleFreshIngredients := 0
	for _, ingredientRange := range FuseRangesUntilAllDisjoint(ingredientRanges) {
		totalPossibleFreshIngredients += ingredientRange.upper - ingredientRange.lower + 1
	}
	fmt.Println("Part 2:", totalPossibleFreshIngredients)
}
