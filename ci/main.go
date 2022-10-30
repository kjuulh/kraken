package main

import (
	"context"
	"log"

	"git.front.kjuulh.io/kjuulh/dagger-go/pkg/builder"
	"git.front.kjuulh.io/kjuulh/dagger-go/pkg/cli"
	"git.front.kjuulh.io/kjuulh/dagger-go/pkg/pipelines"
)

func main() {
	err := cli.NewCustomGoBuild("golangbin", func(ctx context.Context) error {
		builder, err := builder.New(ctx)
		if err != nil {
			return err
		}

		err = pipelines.
			New(builder).
			WithGolangBin(&pipelines.GolangBinOpts{
				DockerImageOpt: &pipelines.DockerImageOpt{
					ImageName: "octopush",
				},
				BuildPath: "cmd/octopush/octopush.go",
				BinName:   "octopush",
			}).
			WithGolangBin(&pipelines.GolangBinOpts{
				DockerImageOpt: &pipelines.DockerImageOpt{
					ImageName: "octopush-server",
				},
				BuildPath: "cmd/server/server.go",
				BinName:   "server",
			}).
			Execute(ctx)

		if err != nil {
			return err
		}

		return nil
	})
	if err != nil {
		log.Fatal(err)
	}
}
