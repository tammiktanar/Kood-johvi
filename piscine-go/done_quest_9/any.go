package anyFunc

func Any(isNumericFunc func(string) bool, arr []string) bool {
	res := false
	for _, check := range arr {
		if isNumericFunc(check) {
			res = true
		}
	}

	return res
}
