package Capitalize

func Capitalize(s string) string {
	res := ""
	firstChar := true
	for _, char := range s {
		if (char >= 65 && char <= 90) || (char >= 97 && char <= 122) || (char >= 48 && char <= 57) {
			if firstChar && (char >= 97 && char <= 122) {
				res += string(char - 32)
			} else {
				if !firstChar && char >= 65 && char <= 90 {
					res += string(char + 32)
				} else {
					res += string(char)
				}
			}
			firstChar = false
		} else {
			firstChar = true
			res += string(char)
		}
	}

	return res
}
