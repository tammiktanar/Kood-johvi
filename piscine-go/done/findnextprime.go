package FindNextPrime

func FindNextPrime(nb int) int {
	res := 0
	if IsPrime(nb) {
		return nb
	} else {
		for ok := true; ok; ok = (res == 0) {
			nb++
			if IsPrime(nb) {
				res = nb
			}
		}
	}
	return res
}

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
