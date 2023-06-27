package printComb

import "github.com/01-edu/z01"

func PrintComb() {
	nr := "0123456789"
	for _, a := range nr {
		for _, b := range nr {
			for _, c := range nr {
				if a < b && b < c {
					z01.PrintRune(a)
					z01.PrintRune(b)
					z01.PrintRune(c)
					if a < '7' {
						z01.PrintRune(',')
						z01.PrintRune(' ')
					}
				}
			}
		}
	}
	z01.PrintRune('\n')
}
