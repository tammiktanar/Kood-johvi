package countIf

func CountIf(isNumericFunc func(string) bool, arr []string) int {
	res := 0
	for _, check := range arr {
		if isNumericFunc(check) {
			res += 1
		}
	}

	return res
}
