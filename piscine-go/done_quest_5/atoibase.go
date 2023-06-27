package piscine

func AtoiBase(s string, baseString string) int {
	base := []rune(baseString)
	if !isValidBase(base) {
		return 0
	}

	slc := []rune(s)
	var result int
	for i := len(slc) - 1; i >= 0; i-- {
		pow := (len(slc) - 1) - i
		num := findRuneValue(slc[i], base)
		result += num * iterativePower(len(base), pow)
	}
	return result
}

func findRuneValue(r rune, base []rune) int {
	for i, v := range base {
		if r == v {
			return i
		}
	}
	return -1
}

func isValidBase(base []rune) bool {
	if len(base) < 2 {
		return false
	}
	for i := range base {
		if base[i] == '-' || base[i] == '+' {
			return false
		}
		for j := i + 1; j < len(base); j++ {
			if base[i] == base[j] {
				return false
			}
		}
	}
	return true
}

func iterativePower(num, exp int) int {
	if exp < 0 {
		return 0
	}
	var total int = 1
	for i := 0; i < exp; i++ {
		total *= num
	}
	return total
}
