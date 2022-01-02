package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	total := 0
	fmt.Println("vim-go")
	file, err := os.Open("2.input")
	if err != nil {
		fmt.Println("vim-go")
	}
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		input := scanner.Text()
		inputs := strings.Split(input, "x")
		intInputs := []int{}
		for _, i := range inputs {
			j, err := strconv.Atoi(i)
			if err != nil {
				panic(err)
			}
			intInputs = append(intInputs, j)
		}
		smallest := intInputs[0] // set the smallest number to the first element of the list
		index := 0
		for i := 0; i < 3; i++ { // iterate over the rest of the list
			num := intInputs[i]
			if num < smallest { // if num is smaller than the current smallest number
				smallest = num // set smallest to num
				index = i
			}
		}
		secondSmallest := 0
		if intInputs[(index+1)%3] > intInputs[(index+2)%3] {
			secondSmallest = intInputs[(index+2)%3]
		} else {
			secondSmallest = intInputs[(index+4)%3]
		}
		sum := 2*intInputs[0]*intInputs[1] + 2*intInputs[0]*intInputs[2] + 2*intInputs[1]*intInputs[2] + smallest*secondSmallest
		total += sum
	}
	fmt.Println("Total: " + strconv.Itoa(total))
}
