package main

import (
	"context"
	"log"

	"git.front.kjuulh.io/kjuulh/dagger-go/pkg/cli"
	"github.com/spf13/cobra"
)

func main() {
	cmd := &cobra.Command{
		Use: "dagger-go",
	}

	cmd.AddCommand(cli.Build(
		func(cmd *cobra.Command) {},
		func(ctx context.Context) error {
			return nil
		}))

	if err := cmd.Execute(); err != nil {
		log.Fatal("%w", err)
	}
}
