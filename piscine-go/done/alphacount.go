package AlphaCount

func AlphaCount(s string) int {
	res := 0
	for _, char := range s {
		if (char >= 65 && char <= 90) || (char >= 97 && char <= 122) {
			res++
		}
	}
	return res
}
