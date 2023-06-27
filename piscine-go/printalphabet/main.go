package main

import "github.com/01-edu/z01"

// Print alphabet
func main() {
	alphabet := "abcdefghijklmnopqrstuvwxyz"
	for _, letter := range alphabet {
		z01.PrintRune(letter)
	}
	z01.PrintRune('\n')
}
