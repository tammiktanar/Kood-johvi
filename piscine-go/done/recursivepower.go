package RecursivePower

func RecursivePower(nb int, power int) int {
	res := 1
	if power < 0 {
		return 0
	} else if power == 0 {
		return 1
	}

	if power == 1 {
		res = nb
	} else {
		res = nb * RecursivePower(nb, (power-1))
	}

	return res
}
