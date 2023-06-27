package advancedSortWordArr

func AdvancedSortWordArr(arr []string, f func(a, b string) int) {
	for {
		ready := true
		for i := 0; i < len(arr)-1; i++ {
			if f(arr[i], arr[i+1]) > 0 {
				ready = false
				buffer := arr[i]
				arr[i] = arr[i+1]
				arr[i+1] = buffer
			}
		}
		if ready {
			break
		}
	}
}
