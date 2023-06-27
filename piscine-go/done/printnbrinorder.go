package PrintNbrInOrder

import "github.com/01-edu/z01"

func PrintNbrInOrder(n int) {
	if n != 0 {
		var nrArray []int
		nrGot := false
		for i := 0; i < n; i++ {
			if !nrGot {
				nrArray = append(nrArray, (n / RecursivePower(10, i) % 10))
				testNr := 0
				for k := (len(nrArray) - 1); k >= 0; k-- {
					testNr = testNr*10 + nrArray[k]
				}

				if testNr == n {
					nrGot = false
					break
				}
			}
		}
		resArray := SortIntegerTable(nrArray)

		for i := 0; i < len(resArray); i++ {
			PrintNbr(resArray[i])
		}
	} else {
		PrintNbr(0)
	}
}

func SortIntegerTable(arr []int) []int {
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
			}
			res[i] = smolInt
			allNr = remove(allNr, allNrIndex)
		}
	}

	return res
}

func remove(slice []int, s int) []int {
	return append(slice[:s], slice[s+1:]...)
}

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

func PrintNbr(n int) {
	t := 1
	if n < 0 {
		t = -1
		z01.PrintRune('-')
	}
	if n != 0 {
		f := (n / 10) * t
		if f != 0 {
			PrintNbr(f)
		}
		k := (n % 10 * t) + '0'
		z01.PrintRune(rune(k))
	} else {
		z01.PrintRune('0')
	}
}
