package main

import (
	"log"

	"git.front.kjuulh.io/kjuulh/dagger-go/pkg/cli"
	"github.com/spf13/cobra"
)

func main() {
	cmd := &cobra.Command{
		Use: "dagger-go",
	}

	cmd.AddCommand(cli.Build("cmd/octopush/octopush.go", ""))
	if err := cmd.Execute(); err != nil {
		log.Fatal("%w", err)
	}
}
