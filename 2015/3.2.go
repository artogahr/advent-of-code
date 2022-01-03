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
	presentCount := 2
	santasTurn := true
	xSanta := 0
	ySanta := 0
	xRobot := 0
	yRobot := 0
	file, _ := os.Open("3.input")
	scanner := bufio.NewReader(file)
	visitedCoordsSanta := []coord{{0, 0}}
	visitedCoordsRobot := []coord{{0, 0}}
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
				if santasTurn {
					xSanta--
					tempCoord = coord{xSanta, ySanta}
				} else {
					xRobot--
					tempCoord = coord{xRobot, yRobot}
				}
			}
		case 62:
			{ //right
				if santasTurn {
					xSanta++
					tempCoord = coord{xSanta, ySanta}
				} else {
					xRobot++
					tempCoord = coord{xRobot, yRobot}
				}
			}
		case 118:
			{ //down
				if santasTurn {
					ySanta++
					tempCoord = coord{xSanta, ySanta}
				} else {
					yRobot++
					tempCoord = coord{xRobot, yRobot}
				}
			}
		case 94:
			{ //up
				if santasTurn {
					ySanta--
					tempCoord = coord{xSanta, ySanta}
				} else {
					yRobot--
					tempCoord = coord{xRobot, yRobot}
				}
			}
		default:
			{
				continue
			}
		}
		found := false
		if santasTurn {
			for i := 0; i < len(visitedCoordsSanta); i++ {
				if visitedCoordsSanta[i].x == xSanta && visitedCoordsSanta[i].y == ySanta {
					found = true
				}
			}
			if !found {
				visitedCoordsSanta = append(visitedCoordsSanta, tempCoord)
				presentCount++
			}
		} else {
			for i := 0; i < len(visitedCoordsRobot); i++ {
				if visitedCoordsRobot[i].x == xRobot && visitedCoordsRobot[i].y == yRobot {
					found = true
				}
			}
			if !found {
				visitedCoordsRobot = append(visitedCoordsRobot, tempCoord)
				presentCount++
			}

		}
		santasTurn = !santasTurn
	}
	for i := 0; i < len(visitedCoordsRobot); i++ {
		for j := 0; j < len(visitedCoordsSanta); j++ {
			if visitedCoordsRobot[i].x == visitedCoordsSanta[j].x && visitedCoordsRobot[i].y == visitedCoordsSanta[j].y {
				presentCount--
			}
		}
	}
	fmt.Print("Present count: ")
	fmt.Println(presentCount)
}
