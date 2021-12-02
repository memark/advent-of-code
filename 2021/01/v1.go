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

func part1() {
	// filename := "example.txt"	// 7
	filename := "input.txt" // 1564
	bytes, _ := ioutil.ReadFile(filename)

	lines := strings.Split(string(bytes), "\n")

	c := 0
	var last int
	for _, line := range lines {
		curr, _ := strconv.Atoi(line)
		if curr > last && last > 0 {
			c++
		}
		last = curr
	}

	fmt.Println(c)
}

func part2() {
	// filename := "example.txt" // 5
	filename := "input.txt" // 1611
	bytes, _ := ioutil.ReadFile(filename)

	lines := strings.Split(string(bytes), "\n")

	c := 0
	w1 := 0
	w2 := 0
	w3 := 0
	lastSum := 0

	for i := range lines {
		if i < 2 {
			continue
		}

		w1, _ = strconv.Atoi(lines[i-2])
		w2, _ = strconv.Atoi(lines[i-1])
		w3, _ = strconv.Atoi(lines[i-0])

		currSum := w1 + w2 + w3
		if currSum > lastSum && lastSum > 0 {
			c++
		}

		lastSum = currSum
	}

	fmt.Println(c)
}
