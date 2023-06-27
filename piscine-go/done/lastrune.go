package LastRune

func LastRune(s string) rune {
	res := 's'
	for _, char := range s {
		res = char
	}

	return res
}
