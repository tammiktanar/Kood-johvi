package main

import "github.com/01-edu/z01"

// Print numbers from 0 - 9
func main() {
	numbers := "0123456789"
	for _, debiil := range numbers {
		z01.PrintRune(debiil)
	}
	z01.PrintRune('\n')
}
