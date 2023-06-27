package IterativePower

func IterativePower(nb int, power int) int {
	res := 1
	if power < 0 {
		return 0
	}

	for i := 0; i < power; i++ {
		res *= nb
	}

	return res
}
