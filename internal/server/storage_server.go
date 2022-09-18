package server

import (
	"context"
	"time"

	"git.front.kjuulh.io/kjuulh/curre"
	"git.front.kjuulh.io/kjuulh/octopush/internal/serverdeps"
	"go.uber.org/zap"
)

func NewStorageServer(logger *zap.Logger, deps *serverdeps.ServerDeps) curre.Component {
	storage := deps.GetStorageService()
	return curre.NewFunctionalComponent(&curre.FunctionalComponent{
		InitFunc: func(_ *curre.FunctionalComponent, ctx context.Context) error {
			logger.Debug("Initializing storage")
			return storage.InitializeStorage(ctx)
		},
		StartFunc: func(fc *curre.FunctionalComponent, ctx context.Context) error {
			return nil
		},
		StopFunc: func(_ *curre.FunctionalComponent, ctx context.Context) error {
			logger.Debug("Cleaning up storage")
			ctx, _ = context.WithTimeout(ctx, time.Second*10)
			return storage.CleanupStorage(ctx)
		},
	})
}
