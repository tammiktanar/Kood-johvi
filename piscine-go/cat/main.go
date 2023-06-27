package main

import (
	"io"
	"os"

	"github.com/01-edu/z01"
)

func main() {
	arguments := os.Args[1:]
	errorString := "ERROR: "
	if len(arguments) == 0 {
		file, err := io.ReadAll(os.Stdin)
		if err != nil {
			for _, char := range string(errorString + err.Error()) {
				z01.PrintRune(char)
			}
			z01.PrintRune('\n')
			os.Exit(1)
		}
		for _, char := range string(file) {
			z01.PrintRune(char)
		}
		return
	}
	for _, filename := range arguments {
		file, err := os.ReadFile(filename)
		if err != nil {
			for _, char := range string(errorString + err.Error()) {
				z01.PrintRune(char)
			}
			z01.PrintRune('\n')
			os.Exit(1)
		}
		for _, char := range string(file) {
			z01.PrintRune(char)
		}
	}
}
