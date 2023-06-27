package sortWordArr

func SortWordArr(arr []string) {
	for {
		ready := true
		for i := 0; i < len(arr)-1; i++ {
			if arr[i] > arr[i+1] {
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
