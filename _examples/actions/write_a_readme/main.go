package main

import "github.com/bitfield/script"

func main() {
	script.
		Echo("# Readme").
		WriteFile("README.md")
}
