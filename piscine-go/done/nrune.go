package NRune

func NRune(s string, n int) rune {
	res := '\x00'
	if n > 0 && n <= len(s) {
		for i, char := range s {
			if i == (n - 1) {
				res = char
			}
		}
	}
	return res
}
