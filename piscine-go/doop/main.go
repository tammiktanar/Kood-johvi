package main

import (
	"os"
)

func main() {
	arguments := os.Args[1:]

	moduloText := "No modulo by 0\n"
	divisionText := "No division by 0\n"

	keepGoing := false
	isNegative := false

	if len(arguments) == 3 {
		if checkIfnumber(arguments[0]) && checkIfnumber(arguments[2]) {
			a := Atoi(arguments[0])
			operator := arguments[1]
			b := Atoi(arguments[2])

			var res int

			if check(a) && check(b) {
				switch operator {
				case "/":
					if b == 0 {
						os.Stderr.WriteString(divisionText)
					} else {
						keepGoing = true
						res = a / b
					}
				case "*":
					keepGoing = true
					res = a * b
				case "-":
					keepGoing = true
					res = a - b
				case "+":
					keepGoing = true
					res = a + b
				case "%":
					if b == 0 {
						os.Stderr.WriteString(moduloText)
					} else {
						keepGoing = true
						res = a % b
					}

				}
				if keepGoing {
					if check(res) {
						resString := string(res%10 + 48)
						k := 10
						if res < 0 {
							isNegative = true
							res = res * -1
							resString = string(res%10 + 48)
						}
						for i := 1; i < k; i++ {
							if Atoi(resString) != res {
								resString = string(res/RecursivePower(10, i)%10+48) + resString
							} else {
								resString += "\n"
								if isNegative {
									resString = "-" + resString
								}
								i = k
							}
						}
						os.Stderr.WriteString(resString)
					}
				}
			}
		}
	}
}

func check(nr int) bool {
	res := false
	if nr < 9223372036854775807 && nr > -9223372036854775807 {
		res = true
	}

	return res
}

func checkIfnumber(s string) bool {
	res := true
	for _, char := range s {
		if !(char >= 48 && char <= 57) {
			res = false
		}
	}
	return res
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

func RecursivePower(nb int, power int) int {
	res := 1
	if power < 0 {
		return 0
	} else if power == 0 {
		return 1
	}

	if power == 1 {
		res = nb
	} else {
		res = nb * RecursivePower(nb, (power-1))
	}

	return res
}
