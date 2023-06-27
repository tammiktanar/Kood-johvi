package ToUpper

func ToUpper(s string) string {
	res := ""
	for _, char := range s {
		if char >= 97 && char <= 122 {
			res += string(char - 32)
		} else {
			res += string(char)
		}
	}

	return res
}
