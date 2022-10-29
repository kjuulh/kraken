package main

import (
	"context"
	"log"

	"git.front.kjuulh.io/kjuulh/dagger-go/pkg/cli"
	"github.com/spf13/cobra"
)

func main() {
	if err := cli.Build(
		func(cmd *cobra.Command) {},
		func(ctx context.Context) error {
			return nil
		}); err != nil {
		log.Fatal(err)
	}
}
