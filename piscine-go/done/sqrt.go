package Sqrt

func Sqrt(nb int) int {
	res := 0
	if nb < 0 {
		return 0
	} else if nb == 1 {
		return 1
	} else {
		i := 1
		for ok := true; ok; ok = !(res >= nb) {
			i++
			res = i * i
		}
		if res == nb {
			return i
		} else {
			return 0
		}
	}
}
