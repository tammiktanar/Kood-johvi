package ToLower

func ToLower(s string) string {
	res := ""
	for _, char := range s {
		if char >= 65 && char <= 90 {
			res += string(char + 32)
		} else {
			res += string(char)
		}
	}

	return res
}
