package FirstRune

func FirstRune(s string) rune {
	res := 's'
	for _, char := range s {
		res = char
		break
	}

	return res
}
