package mapFunc

func Map(f func(int) bool, arr []int) []bool {
	res := make([]bool, len(arr))

	for k, v := range arr {
		res[k] = f(v)
	}
	return res
}
