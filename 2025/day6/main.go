package main

import (
	_ "embed"
	"errors"
	"fmt"
	"strconv"
	"strings"
)

//go:embed input
var data string

type Problem struct {
	numbers []int
	op      string
}

func MakeProblem(op string) Problem {
	return Problem{[]int{}, op}
}

func (p *Problem) AddNumber(number int) {
	p.numbers = append(p.numbers, number)
}

func (p *Problem) Solve() int {
	if p.op == "*" {
		return MulSlice(p.numbers)
	}
	return SumSlice(p.numbers)
}

func SumSlice(numbers []int) int {
	res := 0
	for _, number := range numbers {
		res += number
	}
	return res
}

func MulSlice(numbers []int) int {
	res := 1
	for _, number := range numbers {
		res *= number
	}
	return res
}

func CollectProblemsFromSimpleWorksheet(data string) []Problem {
	lines := strings.Split(data, "\n")
	var problems []Problem

	ops := ParseOps(lines[len(lines)-1])
	numbers := CollectNumbers(lines[:len(lines)-1])

	for problemIndex := range len(numbers[0]) {
		problems = append(problems, MakeProblem(ops[problemIndex]))

		for _, number := range numbers {
			problems[problemIndex].AddNumber(number[problemIndex])
		}
	}

	return problems
}

func CollectNumbers(lines []string) [][]int {
	var buf [][]int

	for _, line := range lines {
		var lineBuf []int

		for item := range strings.SplitSeq(line, " ") {
			if len(item) > 0 {
				res, _ := strconv.Atoi(item)
				lineBuf = append(lineBuf, res)
			}
		}

		buf = append(buf, lineBuf)
	}

	return buf
}

func CollectProblemsFromWeirdWorksheet(data string) []Problem {
	lines := strings.Split(data, "\n")

	problemIndex := 0
	ops := ParseOps(lines[len(lines)-1])
	numbers := lines[:len(lines)-1]

	numColumns := len(numbers[0])
	problems := []Problem{MakeProblem(ops[problemIndex])}

	for columnIndex := range numColumns {
		number, err := CollectColumn(columnIndex, numbers)
		if err == nil {
			problems[problemIndex].AddNumber(number)

		} else {
			problemIndex += 1
			problems = append(problems, MakeProblem(ops[problemIndex]))
		}
	}

	return problems
}

func ParseOps(rawOps string) []string {
	ops := strings.Split(rawOps, " ")
	var buf []string

	for _, op := range ops {
		if len(op) > 0 {
			buf = append(buf, strings.TrimSpace(op))
		}
	}

	return buf
}

func CollectColumn(idx int, numbers []string) (int, error) {
	var buf []byte
	for _, line := range numbers {
		buf = append(buf, line[idx])
	}

	res := strings.TrimSpace(string(buf))
	if res == "" {
		return 0, errors.New("empty column")
	}

	return strconv.Atoi(res)
}

func SolveWorksheet(problems []Problem) int {
	res := 0

	for _, p := range problems {
		res += p.Solve()
	}

	return res
}

func main() {
	fmt.Println("Part 1:", SolveWorksheet(CollectProblemsFromSimpleWorksheet(data)))
	fmt.Println("Part 2:", SolveWorksheet(CollectProblemsFromWeirdWorksheet(data)))
}
