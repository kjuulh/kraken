package main

import (
	"os"

	"git.front.kjuulh.io/kjuulh/kraken/cmd/server/commands"
)

func main() {
	Execute()
}

func Execute() {
	err := commands.CreateServerCmd().Execute()
	if err != nil {
		os.Exit(1)
	}
}
