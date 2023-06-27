package printcomb

import "github.com/01-edu/z01"

func PrintComb2() {
	var notFirst bool
	for i1 := '0'; i1 <= '9'; i1++ {
		for i2 := '0'; i2 <= '9'; i2++ {
			for k1 := '0'; k1 <= '9'; k1++ {
				for k2 := '0'; k2 <= '9'; k2++ {
					if i1 < k1 || (i1 == k1 && i2 < k2) {
						if notFirst {
							z01.PrintRune(',')
							z01.PrintRune(' ')
						} else {
							notFirst = true
						}
						z01.PrintRune(i1)
						z01.PrintRune(i2)
						z01.PrintRune(' ')
						z01.PrintRune(k1)
						z01.PrintRune(k2)
					}
				}
			}
		}
	}
	z01.PrintRune('\n')
}
