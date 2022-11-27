package main

import (
	"os"

	"github.com/bitfield/script"
)

func main() {
	_, err := script.
		Echo("# Readme").
		WriteFile("README.md")

	if err != nil {
		panic(err)
	}
	println("ran stuff")
	entries, err := os.ReadDir(".")
	if err != nil {
		panic(err)
	}
	for _, entry := range entries {
		if !entry.IsDir() {
			file, err := os.ReadFile(entry.Name())
			if err != nil {
				panic(err)
			}
			println(string(file))
		}
	}

	wd, err := os.Getwd()
	if err != nil {
		panic(err)
	}

	println(wd)
}
