package main

import (
	"fmt"
	"io/ioutil"
	"os"
)

func main() {
	tooMany := "Too many arguments"
	notEnough := "File name missing"
	arguments := os.Args[1:]
	if len(arguments) == 1 {
		content, err := ioutil.ReadFile(arguments[0])
		if err != nil {
			fmt.Printf("The mistake is : %v\n", err.Error())
		} else {
			fmt.Print(string(content))
		}
	} else if len(arguments) < 1 {
		fmt.Println(notEnough)
	} else {
		fmt.Println(tooMany)
	}
}
