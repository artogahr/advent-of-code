package main

import (
	"bufio"
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func main() {
	filename := "1.input"
	filebuffer, err := ioutil.ReadFile(filename)
	if err != nil {
		os.Exit(1)
	}
	var floor = 0
	var i = 0
	inputdata := string(filebuffer)
	data := bufio.NewScanner(strings.NewReader(inputdata))
	data.Split(bufio.ScanRunes)
	for data.Scan() {
		i += 1
		var b = data.Text()
		if b == ")" {
			floor -= 1
		} else if b == "(" {
			floor += 1
		} else {
			fmt.Println("I got the byte", b, "("+b+")")
			break
		}
		if floor == -1 {
			fmt.Println("Got to floor -1 in input: ", i)
		}
	}
	fmt.Printf("Final floor: %d\n", floor)
}
