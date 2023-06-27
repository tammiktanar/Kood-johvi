package IsPrime

func IsPrime(nr int) bool {
	res := true
	if nr <= 1 {
		res = false
	} else if nr == 2 {
		res = true
	} else {
		for i := 2; i*i <= nr; i++ {
			if nr%i == 0 {
				return false
			}
		}
	}

	return res
}
