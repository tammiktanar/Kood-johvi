package main

import (
	"os"

	"github.com/01-edu/z01"
)

func main() {
	arguments := os.Args
	arr := []rune(arguments[0])
	res := ""
	for i := 0; i < len(arr); i++ {
		if arr[i] == '/' {
			res = ""
		} else {
			res += string(arr[i])
		}
	}
	for _, char := range res {
		z01.PrintRune(char)
	}
	z01.PrintRune('\n')
}
