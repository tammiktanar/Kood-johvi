package main

import (
	"fmt"
	"os"

	"github.com/01-edu/z01"
)

func main() {
	argHelp := false
	argOrder := false
	argInsert := false
	insertString := "--insert="
	insertStringSmall := "-i="
	insertRune := []rune(insertString)
	inserRuneSmall := []rune(insertStringSmall)
	res := ""
	tempRes := ""
	arguments := os.Args
	if len(arguments) >= 2 {
		for i := 1; i < len(arguments); i++ {
			switch arguments[i] {
			case "--help":
				argHelp = true
			case "-h":
				argHelp = true
			case "--order":
				argOrder = true
			case "-o":
				argOrder = true
			default:
				if !argInsert {
					finished := false
					for l, char := range arguments[i] {
						if len(arguments[i]) >= 4 {
							if !finished {
								if insertRune[l] == char && len(arguments[i]) >= 8 {
									if char == '=' {
										argInsert = true
										finished = true
									}
								} else if len(arguments[i]) >= 4 && inserRuneSmall[l] == char {
									if char == '=' {
										argInsert = true
										finished = true
									}
								} else {
									break
								}
							} else {
								if finished {
									tempRes += string(char)
								}
							}
						}
					}
					if !finished {
						res += arguments[i]
					}
				} else {
					res += arguments[i]
				}
			}
		}
	} else {
		argHelp = true
	}

	if !argHelp {
		if argInsert {
			res += tempRes
		}
		if argOrder {
			runeRes := []rune(res)
			runeRes = SortRuneTable(runeRes)
			for _, char := range runeRes {
				z01.PrintRune(char)
			}
		} else {
			runeRes := []rune(res)
			for _, char := range runeRes {
				z01.PrintRune(char)
			}
		}
		z01.PrintRune('\n')
	} else {
		fmt.Println("--insert")
		fmt.Println("  -i")
		fmt.Println("	 This flag inserts the string into the string passed as argument.")
		fmt.Println("--order")
		fmt.Println("  -o")
		fmt.Println("	 This flag will behave like a boolean, if it is called it will order the argument.")
	}
}

func SortRuneTable(arr []rune) []rune {
	allNr := arr
	res := make([]rune, len(arr))
	for i := 0; i < len(res); i++ {
		res[i] = 0
	}

	for i := 0; i < len(res); i++ {
		if len(allNr) > 0 {
			smolInt := allNr[0]
			allNrIndex := rune(0)

			for k := 0; k < len(allNr); k++ {
				if allNr[k] < smolInt {
					smolInt = allNr[k]
					allNrIndex = rune(k)
				}
			}
			res[i] = smolInt
			allNr = remove(allNr, allNrIndex)
		}
	}

	return res
}

func remove(slice []rune, s rune) []rune {
	return append(slice[:s], slice[s+1:]...)
}
