package main

import (
	"fmt"
	"os"
	"os/exec"
	"strings"
)

/*
	Output - finished
	Reverse - finished
	Fs - finished

	Justify - justify needs to be finished
	Color - Need to add color with setting set words, every x number, the x character
*/

//Global variables
var ResetColor = "\033[0m"
var Bold = "\033[1m"
var Red = "\033[31m"
var Green = "\033[32m"
var Yellow = "\033[33m"
var Blue = "\033[34m"
var Purple = "\033[35m"
var Cyan = "\033[36m"
var Gray = "\033[37m"
var White = "\033[97m"

func main() {
	arguments := os.Args[1:]
	//errorString := "ERROR: "
	if len(arguments) != 0 {
		// normal functionality variables
		arg := arguments      // arguments gotten from the command line
		res := ""             // Resulting string to output
		letters := []string{} // Array where all letters are stored

		letter := ""   // String where to store the current letter
		tempWord := "" // String where to contain word to remove space from the start of it

		wordToPrint := []string{} // The word to print
		typeToPrint := ""         // In what style to print the result in

		// Options related variables
		colored := false                 // Wether colored option was selected
		coloredMulti := false            // If colors has multiple colors chosen
		coloredMultiColors := []string{} // Colors chosen
		coloredPick := ""                // what colored was chosen

		justify := false  // Wether align option was selected
		justifyType := "" // What align option was selected
		width, height, _ := getTermDim()

		reverse := false      // Wether reverse option was selected
		reverseFileName := "" // What's the file to read from

		output := false      // Wether output option was selected
		outputFilename := "" // What's the output file

		devFileName := "dev.txt" // devemoplment mode file

		shouldContinue := true

		for i, curArg := range arg {
			switch i {
			case 0:
				wordToPrint = strings.Split(curArg, "\\n")
				for k, word := range wordToPrint {
					tempWord = ""
					for i, char := range word {
						if (i == 0 || i == len(word)-1) && char == ' ' {
						} else {
							tempWord += string(char)
						}
						if i == len(word)-1 {
							wordToPrint[k] = tempWord
						}
					}
				}
			case 1:
				typeToPrint = curArg
			default:
				if strings.Contains(curArg, "--") {
					if strings.Contains(curArg, "=") {
						optionArr := strings.Split(strings.Split(curArg, "--")[1], "=")
						switch optionArr[0] {
						case "color":
							coloredPick = optionArr[1]
							if strings.Contains(coloredPick, "[") && strings.Contains(coloredPick, "]") {
								coloredMultiColors = strings.Split(strings.Split(strings.Split(curArg, "[")[1], "]")[0], ",")
								coloredMulti = true

							} else {
								colored = true
							}
						case "align":
							justify = true
							justifyType = optionArr[1]
						case "output":
							output = true
							outputFilename = optionArr[1]
						case "reverse":
							reverse = true
							reverseFileName = optionArr[1]
						case "dev":
							devFileName = optionArr[1]

							wordToPrint, shouldContinue = devMode(devFileName, optionArr[1], shouldContinue, wordToPrint, tempWord)

						default:
							fmt.Println(string("Usage: go run . [STRING] [BANNER] [OPTION]"))
							shouldContinue = false
						}
					} else {
						optionCheck := strings.Split(curArg, "--")[1]
						if "dev" == optionCheck {
							wordToPrint, shouldContinue = devMode(devFileName, devFileName, shouldContinue, wordToPrint, tempWord)
						}
					}
				}
			}

		}

		if shouldContinue {
			switch typeToPrint {
			case "thinkertoy":
				filename := "thinkertoy.txt"
				file, err := os.ReadFile(filename)
				if err != nil {
					fmt.Println(string("Usage: go run . [STRING] [BANNER]"))
					shouldContinue = false
					break
				}

				fileContents := string(file)
				for _, char := range fileContents {
					if char != '\r' {
						if char == '\n' {
							char = ' '
						}
						letter += string(char)
					} else {
						letter += " "
						letters = append(letters, letter)
						letter = ""
					}
				}
			case "standard":
				filename := "standard.txt"
				file, err := os.ReadFile(filename)
				if err != nil {
					fmt.Println(string("Usage: go run . [STRING] [BANNER]\n\nEX: go run . something standard"))
					shouldContinue = false
					break
				}

				fileContents := string(file)
				for _, char := range fileContents {
					if char != '\n' {
						letter += string(char)
					} else {
						letters = append(letters, letter)
						letter = ""
					}
				}
			case "shadow":
				filename := "shadow.txt"
				file, err := os.ReadFile(filename)
				if err != nil {
					fmt.Println(string("Usage: go run . [STRING] [BANNER]\n\nEX: go run . something standard"))
					shouldContinue = false
					break
				}

				fileContents := string(file)
				for _, char := range fileContents {
					if char != '\n' {
						letter += string(char)
					} else {
						letter += " "
						letters = append(letters, letter)
						letter = ""
					}
				}
			default:
				fmt.Println(string("Usage: go run . [STRING] [BANNER]\n\nEX: go run . something standard"))
				shouldContinue = false
			}
		}
		if shouldContinue {
			if !reverse { // If reverse is not requested
				for _, words := range wordToPrint { // Get all words
					for i := 1; i <= 8; i++ { // Go through the rows
						tempRes := ""
						for k, char := range words { // Split words into runes
							if int(char)-31 > 0 && int(char) <= 127 { // If the rune is not out of the given character spectrum
								if (((int(char) - 32) * 9) + i) > 0 { // If the character number is bigger than 0
									if shouldContinue {
										if coloredMulti && coloredMultiColors != nil { // If multiple colors were selected
											tempRes += colorize(char, letters[(((int(char)-32)*9)+i)], coloredMultiColors)
										} else { // Else put the rows in 1 character at a time
											tempRes += letters[(((int(char) - 32) * 9) + i)]
										}
									}
									if justify { // I align is set
										switch justifyType { // What align type
										case "center":
										case "left":
										case "right":
										case "justify":
											if k != len(words)-1 {
												if char == ' ' {
													endResLen := 0
													for _, tempWord := range words {
														if string(tempWord) != "" {
															endResLen++
														}
													}
													fmt.Println(words)
													spaces := len(strings.Split(words, " ")) - 1
													nr := ((width - endResLen) - (6 * spaces)) / (len(strings.Split(words, " ")))
													fmt.Println(nr, endResLen)
													for spacesToAdd := 0; spacesToAdd <= nr; spacesToAdd++ {
														tempRes += " "
													}
													if height == 0 {

													}
												}
											}
										default: // If no type was given Error out
											fmt.Println(string("Usage: go run . [STRING] [BANNER] [OPTION]\n\nEX: go run . something standard --align=right"))
											shouldContinue = false
											tempRes = ""
										}
									}
								}
							}
							if k == len(words)-1 {
								if tempRes != "" {
									res += tempRes + "\n"
								}
							}
						}
					}
				}
			} else {
				// Reverse what was given
				if reverseFileName != "" {
					fileReverse, err := os.ReadFile(reverseFileName)
					if err != nil { // If file does not exists Error out
						fmt.Println(string("Usage: go run . [OPTION]\n\nEX: go run . something standard --reverse=<fileName>"))
						shouldContinue = false
					}

					if shouldContinue {

						lettersArrCompact := []string{}

						letterI := 0
						rowNR := 7
						for _, letterRow := range letters { // Turn all letters into a simple character array
							if letterRow != "" {
								switch rowNR {
								case 0:
									for _, char := range letterRow {
										lettersArrCompact[letterI] += string(char)
									}
									letterI++
									rowNR = 8
								case 7:
									for i, char := range letterRow {
										if i == 0 {
											lettersArrCompact = append(lettersArrCompact, string(char))
										} else {
											lettersArrCompact[letterI] += string(char)
										}
									}
								default:
									for _, char := range letterRow {
										lettersArrCompact[letterI] += string(char)
									}
								}
								rowNR--
							}
						}

						if shouldContinue {
							fileContents := string(fileReverse)
							fileConentsArr := []string{}
							comapareVar := ""
							reverseLastI := 0
							characterLength := 1
							fileConentsArr = strings.Split(fileContents, "\n")

							for i := 0; i < (len(fileContents)/8)+8; i++ { // Go through the entire files contents

								if i < (len(fileContents)/8)+8 {
									for j, curLetter := range lettersArrCompact { // go through all letters
										if comapareVar != curLetter { // if it does not match

											if j == len(lettersArrCompact)-1 { // If last letter
												for k := 0; k < 8; k++ { // Go through all rows
													for l := reverseLastI; l < characterLength+reverseLastI; l++ { // The length of the curent attempt of guessing the letter
														if characterLength+reverseLastI >= l {
															if len(fileConentsArr[k]) > l {
																if k == 0 && l == reverseLastI { // If the first character, then empty the array
																	comapareVar = string(fileConentsArr[k][l])
																} else {
																	if string(fileConentsArr[k][l]) != "" {
																		comapareVar += string(fileConentsArr[k][l])
																	}
																}
															}
														}
													}
												}
												characterLength++
											}
										} else {
											reverseLastI = i
											characterLength = 1
											comapareVar = ""
											res += string(j + 32)
										}
									}
								}
							}
						}
						res += "\n"
					}

				} else { // If no reverse file was given Error out
					fmt.Println(string("Usage: go run . [OPTION]\n\nEX: go run . something standard --reverse=<fileName>"))
					shouldContinue = false
				}
			}
		}

		if shouldContinue {
			if !output { // If output argument was not set
				if justify {
					if justifyType == "center" {
						for i := 0; i <= (width - len(res)/8); i++ {
							res = " " + res
							res = strings.Replace(res, "\n", "\n ", 9)
						}
						res += "\n"
					} else if justifyType == "right" {
						expandRes := ""
						for i := 1; i <= (width)-(len(res)/8); i++ {
							expandRes += " "
						}
						newRes := ""
						for i, char := range res {
							if i%(len(res)/8) != 0 {
								newRes += string(char)
							} else {
								newRes += expandRes + string(char)
							}
						}
						res = newRes
					}
				}
				finalizePrint(res, colored, coloredPick) // Print res as is
			} else {
				if outputFilename != "" {
					f, _ := os.Create(outputFilename) // Create the output file if made get it

					f.Truncate(0) // Empty the file contents
					defer f.Close()

					f.WriteString(res) // Write the result into the file
				} else {
					finalizePrint(res, colored, coloredPick)
				}
			}
		}
	}
}

func colorize(letter rune, text string, coloredMultiColors []string) string {
	res := text

	for i, colorArg := range coloredMultiColors {
		if i != 0 {
			for _, char := range colorArg {
				if char == letter {
					switch strings.ToLower(coloredMultiColors[i-1]) {
					case "red":
						res = Red + res + ResetColor
					case "green":
						res = Green + res + ResetColor
					case "yellow":
						res = Yellow + res + ResetColor
					case "blue":
						res = Blue + res + ResetColor
					case "purple":
						res = Purple + res + ResetColor
					case "cyan":
						res = Cyan + res + ResetColor
					case "gray":
						res = Gray + res + ResetColor
					case "white":
						res = White + res + ResetColor
					case "bold":
						res = Bold + res + ResetColor
					}
					break
				}
			}
		}
	}

	return res
}

func finalizePrint(res string, colored bool, coloredPick string) {

	if colored {
		switch strings.ToLower(coloredPick) {
		case "red":
			fmt.Print(Red + res)
		case "green":
			fmt.Print(Green + res)
		case "yellow":
			fmt.Print(Yellow + res)
		case "blue":
			fmt.Print(Blue + res)
		case "purple":
			fmt.Print(Purple + res)
		case "cyan":
			fmt.Print(Cyan + res)
		case "gray":
			fmt.Print(Gray + res)
		case "white":
			fmt.Print(White + res)
		case "bold":
			fmt.Print(Bold + res)
		default:
			fmt.Println("Usage: go run . [STRING] [BANNER] [OPTION]\n\nEX: go run . something standard --color=<color>")
		}
	} else {
		fmt.Print(res)
	}
}

func devMode(devFileName string, optionArr string, shouldContinue bool, wordToPrint []string, tempWord string) ([]string, bool) {

	devFile, err := os.ReadFile(devFileName)
	if err != nil {
		fmt.Println(string("Dev: " + err.Error()))
		shouldContinue = false
	}

	if shouldContinue {
		devFileContents := string(devFile)

		wordToPrint = strings.Split(devFileContents, "\n")
		for k, word := range wordToPrint {
			tempWord = ""
			for i, char := range word {
				if (i == 0 || i == len(word)-1) && char == ' ' {
				} else {
					tempWord += string(char)
				}
				if i == len(word)-1 {
					wordToPrint[k] = tempWord
				}
			}
		}
		fmt.Println(wordToPrint)
	}

	return wordToPrint, shouldContinue
}

func getTermDim() (width, height int, err error) {
	cmd := exec.Command("stty", "size")
	cmd.Stdin = os.Stdin
	var termDim []byte
	if termDim, err = cmd.Output(); err != nil {
		return
	}
	fmt.Sscan(string(termDim), &height, &width)
	return
}
