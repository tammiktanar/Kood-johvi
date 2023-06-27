package foreach

func ForEach(f func(int), arr []int) {
	for _, i := range arr {
		f(i)
	}
}
