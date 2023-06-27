package AppendRange

func AppendRange(min, max int) []int {
	var res []int

	for i := min; i < max; i++ {
		res = append(res, i)
	}

	return res
}
