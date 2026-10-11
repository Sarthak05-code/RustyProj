package main

import (
	"fmt"
	"os"
)

func triple() (result int) {
	defer func() {
		result *= 3
	}()
	return 2
}

func main() {
	name := "Sarthak"
	fmt.Println(triple())
	fmt.Printf("The name is : %s \n" , name)

	f, err := os.Open("file.txt")

	if err != nil {
		fmt.Printf("The file does not exist. ")
	}

	defer f.Close()
}
