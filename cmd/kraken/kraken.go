package main

import (
	"os"

	"git.front.kjuulh.io/kjuulh/kraken/cmd/kraken/commands"
)

func main() {
	Execute()
}

func Execute() {
	err := commands.CreateKrakenCmd().Execute()
	if err != nil {
		os.Exit(1)
	}
}
