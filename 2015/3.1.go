package main

import (
	"bufio"
	"fmt"
	"os"
)

type coord struct {
	x int
	y int
}

func main() {
	presentCount := 1
	x := 0
	y := 0
	file, _ := os.Open("3.input")
	scanner := bufio.NewReader(file)
	visitedCoords := []coord{{0, 0}}
	tempCoord := coord{0, 0}
	for {
		c, _, err := scanner.ReadRune()
		if err != nil {
			break
		}
		// 0------>    +x
		// |
		// |
		// v
		//
		//+y
		switch c {
		case 60:
			{ //left
				x--
				tempCoord = coord{x, y}
			}
		case 62:
			{ //right
				x++
				tempCoord = coord{x, y}
			}
		case 118:
			{ //down
				y++
				tempCoord = coord{x, y}
			}
		case 94:
			{ //up
				y--
				tempCoord = coord{x, y}
			}
		default:
			{
				break
			}
		}
		found := false
		for i := 0; i < len(visitedCoords); i++ {
			if visitedCoords[i].x == x && visitedCoords[i].y == y {
				found = true
			}
		}
		if !found {
			visitedCoords = append(visitedCoords, tempCoord)
			presentCount++
		}
	}
	fmt.Print("Present count: ")
	fmt.Println(presentCount)
}
