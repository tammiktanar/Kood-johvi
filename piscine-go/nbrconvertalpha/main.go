package main

import (
	"os"

	"github.com/01-edu/z01"
)

func main() {
	arguments := os.Args
	isUpper := false

	if len(arguments) >= 2 {
		if arguments[1] == "--upper" || isUpper {
			for i := 2; i < len(arguments); i++ {
				if Atoi(arguments[i]) != 0 && Atoi(arguments[i]) <= 26 && Atoi(arguments[i]) > 0 {
					z01.PrintRune(rune(64 + Atoi(arguments[i])))
				} else {
					z01.PrintRune(' ')
				}
			}
		} else if !isUpper {
			for i := 1; i < len(arguments); i++ {
				if Atoi(arguments[i]) != 0 && Atoi(arguments[i]) <= 26 && Atoi(arguments[i]) > 0 {
					z01.PrintRune(rune(96 + Atoi(arguments[i])))
				} else {
					z01.PrintRune(' ')
				}
			}
		}
		z01.PrintRune('\n')
	}
}

func Atoi(s string) int {
	firstNr := true
	actualNr := true
	isNegative := false
	nr := 0
	for _, char := range s {
		if actualNr {
			if firstNr {
				firstNr = false
				if char != '0' {
					switch char {
					case '1':
						nr = (nr*10 + 1)
					case '2':
						nr = (nr*10 + 2)
					case '3':
						nr = (nr*10 + 3)
					case '4':
						nr = (nr*10 + 4)
					case '5':
						nr = (nr*10 + 5)
					case '6':
						nr = (nr*10 + 6)
					case '7':
						nr = (nr*10 + 7)
					case '8':
						nr = (nr*10 + 8)
					case '9':
						nr = (nr*10 + 9)
					case '-':
						isNegative = true
					}
				}
			} else {
				switch char {
				case '1':
					nr = (nr*10 + 1)
				case '2':
					nr = (nr*10 + 2)
				case '3':
					nr = (nr*10 + 3)
				case '4':
					nr = (nr*10 + 4)
				case '5':
					nr = (nr*10 + 5)
				case '6':
					nr = (nr*10 + 6)
				case '7':
					nr = (nr*10 + 7)
				case '8':
					nr = (nr*10 + 8)
				case '9':
					nr = (nr*10 + 9)
				case '0':
					nr = (nr * 10)
				default:
					nr = 0
					actualNr = false
				}
			}
		}
	}
	if isNegative {
		nr = (nr * (-1))
	}
	return nr
}
