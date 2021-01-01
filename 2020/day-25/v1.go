package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {

	data := `1327981
2822615`

	temp := strings.Split(data, "\n")

	pk1, _ := strconv.Atoi(temp[0])
	pk2, _ := strconv.Atoi(temp[1])

	lz2 := get_loop_size(7, pk2)
	ek1 := get_enc_key(pk1, lz2)

	fmt.Println(ek1)
}

func get_loop_size(sn, pk int) int {
	v := 1
	loop := 0
	for v != pk {
		loop += 1
		v *= sn
		v %= 20201227
	}
	return loop
}

func get_enc_key(pk, lz int) int {
	v := 1
	for i := 0; i < lz; i++ {
		v *= pk
		v %= 20201227
	}
	return v
}
