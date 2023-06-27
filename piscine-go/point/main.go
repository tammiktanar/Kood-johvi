package main

import (
	"github.com/01-edu/z01"
)

type point struct {
	x int
	y int
}

func setPoint(ptr *point) {
	ptr.x = 42
	ptr.y = 21
}

func main() {
	points := &point{}

	setPoint(points)
	var intX int = points.x
	var intY int = points.y
	firstPart := "x = "

	secondPart := ", y = "

	for _, char := range firstPart {
		z01.PrintRune(char)
	}
	z01.PrintRune(rune(intX/10%10 + 48))
	z01.PrintRune(rune(intX%10 + 48))
	for _, char := range secondPart {
		z01.PrintRune(char)
	}
	z01.PrintRune(rune(intY/10%10 + 48))
	z01.PrintRune(rune(intY%10 + 48))
	z01.PrintRune('\n')
}
