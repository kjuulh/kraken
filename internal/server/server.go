package server

import (
	"context"

	"git.front.kjuulh.io/kjuulh/curre"
	"git.front.kjuulh.io/kjuulh/kraken/internal/serverdeps"
	"go.uber.org/zap"
)

func Start(logger *zap.Logger) error {
	ctx := context.Background()

	deps := serverdeps.NewServerDeps(logger)

	return curre.NewManager().
		Register(NewAeroHttpServer(deps)).
		Run(ctx)
}
