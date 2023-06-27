package Compare

func Compare(a, b string) int {
	res := 1
	if a == b {
		res = 0
	} else {
		if a > b {
			res = 1
		} else if b > a {
			res = -1
		}
	}

	return res
}
