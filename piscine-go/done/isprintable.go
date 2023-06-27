package IsPrintable

func IsPrintable(s string) bool {
	res := true

	for _, char := range s {
		if char >= 32 && char <= 126 {
		} else {
			res = false
		}
	}

	return res
}
