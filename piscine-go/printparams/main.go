package main

import (
	"os"

	"github.com/01-edu/z01"
)

func main() {
	arguments := os.Args
	for i := 1; i < len(arguments); i++ {
		arr := []rune(arguments[i])
		for i := 0; i < len(arr); i++ {
			z01.PrintRune(arr[i])
		}
		z01.PrintRune('\n')
	}
}
