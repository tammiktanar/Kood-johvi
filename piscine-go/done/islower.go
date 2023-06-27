package IsLower

func IsLower(s string) bool {
	res := true

	for _, char := range s {
		if char >= 97 && char <= 122 {
		} else {
			res = false
		}
	}

	return res
}
