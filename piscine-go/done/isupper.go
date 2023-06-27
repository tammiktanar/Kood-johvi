package IsUpper

func IsUpper(s string) bool {
	res := true

	for _, char := range s {
		if char >= 65 && char <= 90 {
		} else {
			res = false
		}
	}

	return res
}
