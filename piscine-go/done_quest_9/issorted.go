package issortedFunc

func IsSorted(f func(a, b int) int, arr []int) bool {
	res := 1
	res2 := 1
	res3 := 1
	for k, v := range arr {
		if k != len(arr)-1 {
			if f(v, arr[k+1]) < 0 {
				res++
			}
			if f(v, arr[k+1]) > 0 {
				res2++
			}
			if f(v, arr[k+1]) == 0 {
				res3++
			}
		}
	}
	return res == len(arr) || res2 == len(arr) || res3 == len(arr)
}
