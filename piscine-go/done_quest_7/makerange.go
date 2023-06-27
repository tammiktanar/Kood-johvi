package MakeRange

func MakeRange(min, max int) []int {
	var resNil []int
	if min < max {
		res := make([]int, max-min)
		index := 0
		for i := min; i < max; i++ {
			res[index] = i
			index++
		}

		return res
	}

	return resNil
}
