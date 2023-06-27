// take the strings go through them add every vowel to anarray then reverse the array and then put them back after looping through again
package main

import (
	"os"

	"github.com/01-edu/z01"
)

func main() {
	arguments := os.Args[1:]
	vowelSlice := make([]rune, 0)
	if len(arguments) > 0 {
		for _, word := range arguments {
			for _, char := range word {
				if isVowel(char) == true {
					vowelSlice = append(vowelSlice, char)
				} else {
					char = 'P'
				}
			}
		}
		if len(vowelSlice) > 0 {
			i := len(vowelSlice) - 1
			for _, word := range arguments {
				for _, char := range word {
					if isVowel(char) == true {
						z01.PrintRune(vowelSlice[i])
						i--
					} else {
						z01.PrintRune(char)
					}
				}
				z01.PrintRune(' ')
			}
		} else {
			for _, word := range arguments {
				for _, char := range word {
					z01.PrintRune(char)
				}
				z01.PrintRune(' ')
			}
		}
	}
	z01.PrintRune('\n')
}

func isVowel(char rune) bool {
	res := false
	if char == 'a' || char == 'i' || char == 'u' || char == 'e' || char == 'o' || char == 'A' || char == 'I' || char == 'U' || char == 'E' || char == 'O' {
		res = true
	}
	return res
}
