package cli

import (
	"context"

	"git.front.kjuulh.io/kjuulh/curre"
	"git.front.kjuulh.io/kjuulh/octopush/internal/server"
	"git.front.kjuulh.io/kjuulh/octopush/internal/serverdeps"
	"git.front.kjuulh.io/kjuulh/octopush/internal/services/signer"
	"go.uber.org/zap"
)

func Start(ctx context.Context, logger *zap.Logger) (*serverdeps.ServerDeps, curre.CleanupFunc, error) {
	deps := serverdeps.NewServerDeps(logger)

	readyChan := make(chan curre.ComponentsAreReady, 1)

	cleanupFunc, err := curre.NewManager().
		Register(
			server.NewStorageServer(logger.With(zap.Namespace("storage")), deps),
		).
		Register(
			signer.NewOpenPGPApp(deps.GetOpenPGP()),
		).
		RunNonBlocking(ctx, readyChan)

	<-readyChan

	return deps, cleanupFunc, err
}
