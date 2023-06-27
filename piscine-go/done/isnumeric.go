package IsNumeric

func IsNumeric(s string) bool {
	res := true

	for _, char := range s {
		if char >= 48 && char <= 57 {
		} else {
			res = false
		}
	}

	return res
}
