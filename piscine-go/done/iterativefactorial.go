package IterativeFactorial

func IterativeFactorial(nb int) int {
	res := 1
	if nb < 0 || nb > 32 {
		return 0
	} else {
		for i := nb; i > 0; i-- {
			res *= i
		}
	}

	return res
}
