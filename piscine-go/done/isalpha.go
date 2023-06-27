package IsAlpha

func IsAlpha(s string) bool {
	res := true

	for _, char := range s {
		if (char >= 65 && char <= 90) || (char >= 97 && char <= 122) || (char >= 48 && char <= 57) {
		} else {
			res = false
		}
	}

	return res
}
