package BasicAtoi

func BasicAtoi(s string) int {
	firstNr := true
	nr := 0
	for _, char := range s {
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
			default:
				nr = (nr * 10)
			}
		}
	}

	return nr
}
