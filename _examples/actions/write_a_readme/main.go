package main

import "github.com/bitfield/script"

func main() {
	_, err := script.
		Echo("# Readme").
		WriteFile("README.md")
	if err != nil {
		panic(err)
	}
}
