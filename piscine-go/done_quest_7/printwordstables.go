package PrintWordsTables

import "github.com/01-edu/z01"

func PrintWordsTables(a []string) {
	for i := 0; i < len(a); i++ {
		for _, char := range a[i] {
			z01.PrintRune(char)
		}
		z01.PrintRune('\n')
	}
}
