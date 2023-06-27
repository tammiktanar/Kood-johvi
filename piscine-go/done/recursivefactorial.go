package RecursiveFactorial

func RecursiveFactorial(nb int) int {
	res := 1
	if nb < 0 || nb > 32 {
		return 0
	} else {
		if nb != 0 {
			res = nb * RecursiveFactorial(nb-1)
		}
	}

	return res
}
