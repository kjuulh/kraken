package actions

import (
	"context"

	"git.front.kjuulh.io/kjuulh/kraken/internal/services/storage"
	"go.uber.org/zap"
)

type Predicate func(ctx context.Context, area *storage.Area) (bool, error)
type ActionFunc func(ctx context.Context, area *storage.Area) error

type Action struct {
	logger *zap.Logger
}

func NewAction(logger *zap.Logger) *Action {
	return &Action{logger: logger}
}

func (a *Action) Run(ctx context.Context, area *storage.Area, predicate Predicate, action ActionFunc, dryrun bool) error {
	matches, err := predicate(ctx, area)
	if err != nil {
		return err
	}

	if !matches {
		a.logger.Debug("repo doesn't match, skipping", zap.String("path", area.Path))
		return nil
	}

	if dryrun {
		a.logger.Panic("dryrun selected, but not implemented yet")
		return nil
	}

	err = action(ctx, area)
	if err != nil {
		return err
	}

	return nil
}
