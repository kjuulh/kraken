package server

import (
	"context"

	"git.front.kjuulh.io/kjuulh/curre"
	"git.front.kjuulh.io/kjuulh/kraken/internal/serverdeps"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/signer"
	"go.uber.org/zap"
)

func Start(logger *zap.Logger) error {
	ctx := context.Background()

	deps := serverdeps.NewServerDeps(logger)

	return curre.NewManager().
		Register(NewGinHttpServer(logger.With(zap.Namespace("ginHttpServer")), deps)).
		Register(NewStorageServer(logger.With(zap.Namespace("storageServer")), deps)).
		Register(signer.NewOpenPGPApp(deps.GetOpenPGP())).
		Run(ctx)
}
