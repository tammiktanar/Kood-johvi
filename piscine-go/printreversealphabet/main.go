package main

import "github.com/01-edu/z01"

// Print alphabet in reverse
func main() {
	alphabet := "abcdefghijklmnopqrstuvwxyz"
	for _, debiil := range Reverse(alphabet) {
		z01.PrintRune(debiil)
	}
	z01.PrintRune('\n')
}

func Reverse(s string) (result string) { // https://stackoverflow.com/a/4965535
	for _, v := range s {
		result = string(v) + result
	}
	return
}
