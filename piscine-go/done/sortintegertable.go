package SortIntegerTable

func SortIntegerTable(arr []int) {
	allNr := arr
	res := make([]int, len(arr))
	for i := 0; i < len(res); i++ {
		res[i] = 0
	}

	for i := 0; i < len(res); i++ {
		if len(allNr) > 0 {
			smolInt := allNr[0]
			allNrIndex := 0

			for k := 0; k < len(allNr); k++ {
				if allNr[k] < smolInt {
					smolInt = allNr[k]
					allNrIndex = k
				}
				// fmt.Println(arr[k])
			}
			res[i] = smolInt
			allNr = remove(allNr, allNrIndex)
		}
	}

	for i := 0; i < len(res); i++ {
		arr[i] = res[i]
	}
}

func remove(slice []int, s int) []int {
	return append(slice[:s], slice[s+1:]...)
}
