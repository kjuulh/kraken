package actions

import (
	"context"
	"fmt"
	"os"
	"path"
	"time"

	"git.front.kjuulh.io/kjuulh/kraken/internal/schema"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/providers"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/storage"
	"go.uber.org/zap"
)

type (
	ActionCreatorOps struct {
		RepositoryUrl string
		Branch        string
		Path          string
	}

	ActionCreator struct {
		logger  *zap.Logger
		storage *storage.Service
		git     *providers.Git
	}

	ActionCreatorDeps interface {
		GetStorageService() *storage.Service
		GetGitProvider() *providers.Git
	}
)

func NewActionCreator(logger *zap.Logger, deps ActionCreatorDeps) *ActionCreator {
	return &ActionCreator{
		logger:  logger,
		storage: deps.GetStorageService(),
		git:     deps.GetGitProvider(),
	}
}

func (ac *ActionCreator) Prepare(ctx context.Context, ops *ActionCreatorOps) (*Action, error) {
	area, err := ac.storage.CreateArea(ctx)
	if err != nil {
		ac.logger.Error("failed to allocate area", zap.Error(err))
		return nil, err
	}

	cloneCtx, _ := context.WithTimeout(ctx, time.Second*10)
	_, err = ac.git.CloneBranch(cloneCtx, area, ops.RepositoryUrl, ops.Branch)
	if err != nil {
		ac.logger.Error("could not clone repo", zap.Error(err))
		return nil, err
	}

	executorUrl := path.Join(area.Path, ops.Path)
	if _, err = os.Stat(executorUrl); os.IsNotExist(err) {
		return nil, fmt.Errorf("path is invalid: %s", ops.Path)
	}

	contents, err := os.ReadFile(path.Join(executorUrl, "kraken.yml"))
	if err != nil {
		return nil, err
	}

	krakenSchema, err := schema.Unmarshal(string(contents))
	if err != nil {
		return nil, err
	}

	ac.logger.Debug("Action creator done")
	return &Action{
		Schema:     krakenSchema,
		SchemaPath: executorUrl,
	}, nil
}

func (ac *ActionCreator) Cleanup(ctx context.Context, area *storage.Area) {
	ac.logger.Debug("Removing area", zap.String("path", area.Path))
	err := ac.storage.RemoveArea(ctx, area)
	if err != nil {
		panic(err)
	}
}
