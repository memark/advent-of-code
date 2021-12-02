package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	part1() // 1692075
	part2() // 1749524700
}

func part1() {
	// filename := "example.txt"
	filename := "input.txt"
	bytes, _ := ioutil.ReadFile(filename)
	lines := strings.Split(string(bytes), "\n")
	instructions := parseInstructions(lines)

	depth := 0
	horizontal := 0

	for _, v := range instructions {
		switch v.operation {
		case "forward":
			// forward X increases the horizontal position by X units.
			horizontal += v.operand
		case "down":
			// down X increases the depth by X units.
			depth += v.operand
		case "up":
			// up X decreases the depth by X units.
			depth -= v.operand
		}
	}

	fmt.Println("part1", horizontal*depth, "\t", horizontal, "\t", depth)
}

func part2() {
	// filename := "example.txt"
	filename := "input.txt"
	bytes, _ := ioutil.ReadFile(filename)
	lines := strings.Split(string(bytes), "\n")
	instructions := parseInstructions(lines)

	depth := 0
	horizontal := 0
	aim := 0

	for _, v := range instructions {
		switch v.operation {
		case "down":
			// down X increases your aim by X units.
			aim += v.operand
		case "up":
			// up X decreases your aim by X units.
			aim -= v.operand
		case "forward":
			// forward X does two things:
			// 		It increases your horizontal position by X units.
			// 		It increases your depth by your aim multiplied by X.
			horizontal += v.operand
			depth += aim * v.operand
		}
	}

	fmt.Println("part2", horizontal*depth, "\t", horizontal, "\t", depth)
}

type Instruction struct {
	operation string
	operand   int
}

func parseInstructions(lines []string) []Instruction {
	instructions := []Instruction{}
	for _, l := range lines {
		ss := strings.Split(l, " ")
		n, _ := strconv.Atoi(ss[1])
		i := Instruction{
			operation: ss[0],
			operand:   n,
		}
		instructions = append(instructions, i)
	}
	return instructions
}
