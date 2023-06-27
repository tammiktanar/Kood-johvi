package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	arguments := os.Args[1:]
	errorString := "ERROR: "
	if len(arguments) != 0 {
		arg := arguments
		firstRun := true
		res := []string{}
		for _, filename := range arg {
			if firstRun {
				finalString := []string{}
				word := ""
				prevWord := ""
				commandWord := ""
				file, err := os.ReadFile(filename)
				if err != nil {
					fmt.Println(string(errorString + err.Error()))
					os.Exit(1)
				}

				fileContents := strings.Replace(string(file)+"\n   ", "  ", " ", -1)

				for _, char := range fileContents {
					if char != ' ' {
						if char != ')' {
							switch char {
							case ',':
								word += string(char)
								switch word {
								case "(low,":
									commandWord = "low"
								case "(cap,":
									commandWord = "cap"
								case "(up,":
									commandWord = "up"
								default:
									finalString, prevWord, word = check(prevWord, word, finalString)
									finalString = append(finalString, " ")
								}
							case '\n':
								finalString, prevWord, word = check(prevWord, word, finalString)
								word += string(char)
								finalString = append(finalString, " ")
								finalString, prevWord, word = check(prevWord, word, finalString)
							case '\r':
								word += ""
							default:
								if checkPonctuation(string(char)) {
									finalString, prevWord, word = check(prevWord, word, finalString)
									word += string(char)
									finalString = append(finalString, " ") // This mother fucker has been a pain in the ass to find
									finalString, prevWord, word = check(prevWord, word, finalString)
								} else {
									word += string(char)
								}
							}
						} else {
							if commandWord == "" {
								word += string(char)
								finalString, prevWord, word = check(prevWord, word, finalString)
								finalString = append(finalString, " ")
							} else {
								word += string(char)
								finalString, prevWord, word = checkCommand(prevWord, word, finalString, commandWord)
								commandWord = ""
							}
						}
					} else {
						// add command check here later
						if commandWord == "" {
							finalString, prevWord, word = check(prevWord, word, finalString)
							finalString = append(finalString, " ")
						}
					}
				}

				for i, char := range finalString {
					if i != 0 {
						if char == finalString[i-1] && char == " " {
							finalString[i] = ""
						}
					}
				}

				finalString = removeEmptyStrings(finalString)

				finalString = append(finalString, "\n")
				finalStringPrevWord := ""
				finalStringSendWord := ""
				finalStringWord := ""
				movedRight := false
				movedRightSpecial := false
				start := false
				for i, char := range finalString {
					if !start {
						if char != " " {
							start = true
						}
					}

					if start {
						if finalStringWord == " " {
							switch strings.ToLower(char) {
							case ".":
								finalStringSendWord = finalStringPrevWord
								finalStringPrevWord = "."
								finalStringWord = ""
							case ",":
								finalStringSendWord = finalStringPrevWord
								finalStringPrevWord = ","
								finalStringWord = ""
							case ";":
								finalStringSendWord = finalStringPrevWord
								finalStringPrevWord = ";"
								finalStringWord = ""
							case ":":
								finalStringSendWord = finalStringPrevWord
								finalStringPrevWord = ":"
								finalStringWord = ""
							case "?":
								finalStringSendWord = finalStringPrevWord
								finalStringPrevWord = "?"
								finalStringWord = ""
							case "!":
								finalStringSendWord = finalStringPrevWord
								finalStringPrevWord = "!"
								finalStringWord = ""
							case "a":
								changeTo := char
								if len(finalString)-1 >= i+2 {
									character := strings.ToLower(finalString[i+2])[0]
									if character == 'a' || character == 'e' || character == 'i' || character == 'o' || character == 'u' {
										if char == "A" {
											changeTo = "An"
										} else {
											changeTo = "an"
										}
									}
								}

								finalStringSendWord = finalStringPrevWord
								finalStringPrevWord = finalStringWord
								finalStringWord = changeTo
							default:
								if char == "'" || char == "‘" || char == "’" { // Check apastrophe
									if finalStringWord != " " && finalString[i+1] != " " { // If it has no spaces next to it don't move it
										finalStringSendWord = finalStringPrevWord
										finalStringPrevWord = finalStringWord
										finalStringWord = char
									} else {
										if char == "‘" || char == "’" {
											if !movedRightSpecial {
												char = "‘"
											} else {
												char = "’"
											}
											finalString, finalStringWord, finalStringPrevWord, finalStringSendWord, movedRightSpecial = alterAphostrophe(finalString, i, char, finalStringWord, finalStringPrevWord, movedRightSpecial)

										} else {
											finalString, finalStringWord, finalStringPrevWord, finalStringSendWord, movedRight = alterAphostrophe(finalString, i, char, finalStringWord, finalStringPrevWord, movedRight)
										}
									}

								} else {

									finalStringSendWord = finalStringPrevWord
									finalStringPrevWord = finalStringWord
									finalStringWord = char
								}
							}
						} else {
							finalStringSendWord = finalStringPrevWord
							finalStringPrevWord = finalStringWord
							finalStringWord = char
						}

						if finalStringPrevWord == " " && finalStringSendWord == " " {
							finalStringSendWord = ""
						} else if finalStringPrevWord == " " && finalStringWord == " " {
							finalStringPrevWord = ""
						}

						if finalStringSendWord != "" {
							res = append(res, finalStringSendWord)
						}

						if char == "\n" {
							start = false
						}
						if i == len(finalString) {
							if finalStringSendWord != "" {
								res = append(res, finalStringSendWord)
								if finalStringPrevWord != "" {
									res = append(res, finalStringPrevWord)
									if finalStringWord != "" {
										res = append(res, finalStringWord)
										if char != "" {
											res = append(res, char)
										}
									}
								}
							}
						}
					}
				}

				res = append(res, "\n")

				firstRun = false

			} else {
				f, _ := os.Create(filename)
				defer f.Close()

				for _, char := range res {
					f.WriteString(char)
				}
			}
		}
	} else {
		fmt.Println(string(errorString + "no arguments given\n"))
	}
}

func checkCommand(prevWord string, word string, finalString []string, commandWord string) ([]string, string, string) {

	if prevWord != "" {
		switch commandWord {
		case "up":
			commandString := strings.Trim(strings.Trim(word, "(up,"), ")")

			commandInt, _ := strconv.Atoi(commandString)
			if commandInt > 0 {
				for i := len(finalString); i >= 0; i-- {
					if commandInt > 0 {
						if i == len(finalString) {
							prevWord = strings.ToUpper(prevWord)
							commandInt--
						} else {
							if checkSpecialChar(finalString[i]) {
								finalString[i] = strings.ToUpper(finalString[i])
								commandInt--
							}
						}
					}
				}
			}
		case "low":
			commandString := strings.Trim(strings.Trim(word, "(low,"), ")")
			commandInt, _ := strconv.Atoi(commandString)
			if commandInt > 0 {
				for i := len(finalString); i >= 0; i-- {
					if commandInt > 0 {
						if i == len(finalString) {
							prevWord = strings.ToLower(prevWord)
							commandInt--
						} else {
							if checkSpecialChar(finalString[i]) {
								finalString[i] = strings.ToLower(finalString[i])
								commandInt--
							}
						}
					}
				}
			}
		case "cap":
			commandString := strings.Trim(strings.Trim(word, "(cap,"), ")")
			commandInt, _ := strconv.Atoi(commandString)
			if commandInt > 0 {
				for i := len(finalString); i >= 0; i-- {
					if commandInt > 0 {
						if i == len(finalString) {
							prevWord = strings.Title(strings.ToLower(prevWord))
							commandInt--
						} else {
							if checkSpecialChar(finalString[i]) {
								finalString[i] = strings.Title(strings.ToLower(finalString[i]))
								commandInt--
							}
						}
					}
				}
			}
		}
		word = ""
	} else {
		prevWord = word
		word = ""
	}

	return finalString, prevWord, word
}

func check(prevWord string, word string, finalString []string) ([]string, string, string) {
	if prevWord != "" {
		switch strings.ToLower(word) {
		case "(hex)":

			intPrevWord, err := strconv.ParseInt(hexaNumberToInteger(strings.Trim(prevWord, " ")), 16, 64)
			if err == nil {
				intConvert := strconv.FormatInt(intPrevWord, 10)
				prevWord = intConvert
			}
			word = ""
		case "(bin)":

			intPrevWord, err := strconv.ParseInt(strings.Trim(prevWord, " "), 2, 64)
			if err == nil {
				intConvert := strconv.FormatInt(intPrevWord, 10)
				prevWord = intConvert
			}
			word = ""
		case "(up)":

			prevWord = strings.ToUpper(prevWord)
			word = ""
		case "(low)":

			prevWord = strings.ToLower(prevWord)
			word = ""
		case "(cap)":

			prevWord = strings.Title(strings.ToLower(prevWord))
			word = ""
		default:
			finalString = append(finalString, prevWord)
			prevWord = word
			word = ""
		}
	} else {
		prevWord = word
		word = ""
	}

	return finalString, prevWord, word
}

// Found on https://www.cloudhadoop.com/2018/12/golang-example-convertcast-hexa-to20.html
func hexaNumberToInteger(hexaString string) string {
	// replace 0x or 0X with empty String
	numberStr := strings.Replace(hexaString, "0x", "", -1)
	numberStr = strings.Replace(numberStr, "0X", "", -1)
	return numberStr
}

func checkPonctuation(str string) bool {
	res := false
	switch str {
	case ".":
		res = true
	case ",":
		res = true
	case ";":
		res = true
	case ":":
		res = true
	case "?":
		res = true
	case "!":
		res = true
	case "’":
		res = true
	case "‘":
		res = true
	default:
		res = false
	}
	return res
}

func checkSpecialChar(char string) bool {
	res := false
	if char != " " && char != "," && char != "." && char != "!" && char != "?" && char != ";" && char != ":" && char != "'" && char != "\"" && char != "\n" {
		res = true
	}
	return res
}

func alterAphostrophe(finalString []string, i int, char string, finalStringWord string, finalStringPrevWord string, movedRight bool) ([]string, string, string, string, bool) {
	finalStringSendWord := ""
	if finalStringWord != " " && finalString[i+1] != " " { // If it has no spaces next to it don't move it
		finalStringSendWord = finalStringPrevWord
		finalStringPrevWord = finalStringWord
		finalStringWord = char
	} else {
		if !movedRight { // if it hasn't been move to the left
			if len(finalString)-1 >= i+2 {
				if finalString[i+1] == " " { // check if there is a space to the left
					if finalString[i+2] != " " { // check if there is a word to the left
						finalString[i+1] = "" // remove the space
						movedRight = true
					}
				}
			}
		} else {
			if len(finalString) >= i { // if there are more characters to the left
				if finalStringWord == " " { // if there is some space to the left
					if finalStringPrevWord != " " { // if there is a word to the left
						finalStringWord = "" // remove the space
						movedRight = false
					}
				}
			}
		}

		finalStringSendWord = finalStringPrevWord
		finalStringPrevWord = finalStringWord
		finalStringWord = char
	}

	return finalString, finalStringWord, finalStringPrevWord, finalStringSendWord, movedRight
}

func removeEmptyStrings(s []string) []string {
	var r []string
	for _, str := range s {
		if str != "" {
			r = append(r, str)
		}
	}
	return r
}
