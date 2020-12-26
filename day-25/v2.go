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

	c1 := make(chan int)
	c2 := make(chan int)

	go get_loop_size(7, pk2, c1)
	go get_loop_size(7, pk1, c2)
	lz1, lz2 := <-c1, <-c2

	go get_enc_key(pk1, lz2, c1)
	go get_enc_key(pk2, lz1, c2)
	ek1, ek2 := <-c1, <-c2

	fmt.Println(ek1, ek2)
}

func get_loop_size(sn, pk int, res chan<- int) {
	v := 1
	loop := 0
	for v != pk {
		loop += 1
		v *= sn
		v %= 20201227
	}
	res <- loop
}

func get_enc_key(pk, lz int, res chan<- int) {
	v := 1
	for i := 0; i < lz; i++ {
		v *= pk
		v %= 20201227
	}
	res <- v
}
