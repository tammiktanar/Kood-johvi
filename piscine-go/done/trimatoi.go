package TrimAtoi

func TrimAtoi(s string) int {
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
					default:
						firstNr = true
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
				}
			}
		}
	}
	if isNegative {
		nr = (nr * (-1))
	}
	return nr
}
