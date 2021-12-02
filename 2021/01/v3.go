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
	numbers := atoi(lines)

	count := 0
	for i := windowSize - 1 + 1; i < len(numbers); i++ {
		prevSum := sum(numbers[i-windowSize : i])
		currSum := sum(numbers[i-windowSize+1 : i+1])
		if currSum > prevSum {
			count++
		}
	}

	fmt.Println(count)
}

func sum(numbers []int) int {
	sum := 0
	for _, n := range numbers {
		sum += n
	}
	return sum
}

func atoi(lines []string) []int {
	numbers := make([]int, len(lines))
	for i, l := range lines {
		n, _ := strconv.Atoi(l)
		numbers[i] = n
	}
	return numbers
}
