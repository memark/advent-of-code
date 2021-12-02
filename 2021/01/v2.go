package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	part1()
	part2()
}

func part1() { solve(1) } // 1564

func part2() { solve(3) } // 1611

func solve(windowSize int) {
	// filename := "example.txt"
	filename := "input.txt"
	bytes, _ := ioutil.ReadFile(filename)
	lines := strings.Split(string(bytes), "\n")
	numbers := Atoi(lines)

	c := 0
	lastSum := 0

	for i := range numbers {
		if i < windowSize-1 {
			continue
		}

		currSum := 0
		for j := 0; j < windowSize; j++ {
			currSum += numbers[i-j]
		}

		if currSum > lastSum && lastSum > 0 {
			c++
		}

		lastSum = currSum
	}

	fmt.Println(c)
}

func Atoi(lines []string) []int {
	numbers := make([]int, len(lines))
	for i, l := range lines {
		n, _ := strconv.Atoi(l)
		numbers[i] = n
	}
	return numbers
}
