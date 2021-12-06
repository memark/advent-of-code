package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	// part1() // 5934, 394994
	part2() // 26984457539, ?
}

func part1() {
	fmt.Println("part1", getScore(80))
}

func part2() {
	fmt.Println("part2", getScore(256))
}

func getScore(days int) int {
	filename := "example.txt"
	// filename := "input.txt"
	bytes, _ := ioutil.ReadFile(filename)
	values := strings.Split(string(bytes), ",")
	fish := Atoi(values)

	fishes := fish[:]

	fmt.Println("Initial state:", fishes)

	for day := 1; day <= days; day++ {
		l := len(fishes)

		for i := 0; i < l; i++ {
			if fishes[i] == 0 {
				fishes[i] = 6
				fishes = append(fishes, 8)
			} else {
				fishes[i]--
			}
		}

		fmt.Println("After", day, "days:", len(fishes))
	}

	return len(fishes)
}

func Atoi(lines []string) []int {
	numbers := make([]int, len(lines))
	for i, l := range lines {
		n, _ := strconv.Atoi(l)
		numbers[i] = n
	}
	return numbers
}
