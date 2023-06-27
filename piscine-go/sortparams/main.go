package main

import (
	"os"

	"github.com/01-edu/z01"
)

func main() {
	args := os.Args[1:]
	for { // Bubble sort
		ready := true
		for i := 0; i < len(args)-1; i++ {
			if isStringGreater(args[i], args[i+1]) {
				ready = false
				buffer := args[i]
				args[i] = args[i+1]
				args[i+1] = buffer
			}
		}
		if ready {
			break
		}
	}

	for _, s := range args {
		for _, r := range s {
			z01.PrintRune(r)
		}
		z01.PrintRune('\n')
	}
}

func isStringGreater(aString, bString string) bool {
	aRunes := []rune(aString)
	bRunes := []rune(bString)
	for i := 0; i < min(len(aRunes), len(bRunes)); i++ {
		a, b := aRunes[i], bRunes[i]
		if !(a == b) {
			return a > b
		}
	}
	return len(aRunes) > len(bRunes)
}

func min(a, b int) int {
	if a > b {
		return b
	}
	return a
}
