package commands

import (
	"context"
	"fmt"
	"sync"
	"time"

	"git.front.kjuulh.io/kjuulh/kraken/internal/actions"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/providers"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/storage"
	"go.uber.org/zap"
)

type (
	ProcessRepos struct {
		logger        *zap.Logger
		storage       *storage.Service
		git           *providers.Git
		actionCreator *actions.ActionCreator
	}

	ProcessReposDeps interface {
		GetStorageService() *storage.Service
		GetGitProvider() *providers.Git
		GetActionCreator() *actions.ActionCreator
	}
)

func NewProcessRepos(logger *zap.Logger, deps ProcessReposDeps) *ProcessRepos {
	return &ProcessRepos{
		logger:        logger,
		storage:       deps.GetStorageService(),
		git:           deps.GetGitProvider(),
		actionCreator: deps.GetActionCreator(),
	}
}

func (pr *ProcessRepos) Process(ctx context.Context, repository string, branch string, actionPath string) error {
	errChan := make(chan error, 1)

	action, err := pr.actionCreator.Prepare(ctx, &actions.ActionCreatorOps{
		RepositoryUrl: repository,
		Branch:        branch,
		Path:          actionPath,
	})
	if err != nil {
		return err
	}

	repositoryUrls := make([]string, 0)

	wg := sync.WaitGroup{}
	wg.Add(len(repositoryUrls))

	for _, repoUrl := range repositoryUrls {
		go func(ctx context.Context, repoUrl string) {
			defer func() {
				wg.Done()
			}()
			err := pr.processRepo(ctx, repoUrl, action)
			if err != nil {
				pr.logger.Error("could not process repo", zap.Error(err))
				errChan <- err
			}
		}(ctx, repoUrl)
	}

	wg.Wait()
	close(errChan)
	pr.logger.Debug("finished processing all repos")

	for err := range errChan {
		return err
	}

	return nil
}

func (pr *ProcessRepos) processRepo(ctx context.Context, repoUrl string, action *actions.Action) error {
	cleanup, area, err := pr.prepareAction(ctx)
	defer func() {
		if cleanup != nil {
			cleanup(ctx)
		}
	}()
	if err != nil {
		return err
	}

	repo, err := pr.clone(ctx, area, repoUrl)
	if err != nil {
		return err
	}

	err = action.Execute(ctx, area)
	if err != nil {
		return err
	}

	err = pr.commit(ctx, area, repo)
	if err != nil {
		return err
	}

	pr.logger.Debug("processing done", zap.String("path", area.Path), zap.String("repoUrl", repoUrl))
	return nil
}

func (pr *ProcessRepos) prepareAction(
	ctx context.Context,
) (func(ctx context.Context), *storage.Area, error) {
	pr.logger.Debug("Creating area")
	area, err := pr.storage.CreateArea(ctx)
	if err != nil {
		pr.logger.Error("failed to allocate area", zap.Error(err))
		return nil, nil, err
	}

	cleanupfunc := func(ctx context.Context) {
		pr.logger.Debug("Removing area", zap.String("path", area.Path))
		err = pr.storage.RemoveArea(ctx, area)
		if err != nil {
			panic(err)
		}
	}

	return cleanupfunc, area, nil
}

func (pr *ProcessRepos) clone(ctx context.Context, area *storage.Area, repoUrl string) (*providers.GitRepo, error) {
	pr.logger.Debug("Cloning repo", zap.String("path", area.Path), zap.String("repoUrl", repoUrl))
	cloneCtx, _ := context.WithTimeout(ctx, time.Second*5)
	repo, err := pr.git.Clone(cloneCtx, area, repoUrl)
	if err != nil {
		pr.logger.Error("could not clone repo", zap.Error(err))
		return nil, err
	}

	err = pr.git.CreateBranch(ctx, repo)
	if err != nil {
		pr.logger.Error("could not create branch", zap.Error(err))
		return nil, err
	}

	return repo, nil
}

func (pr *ProcessRepos) commit(ctx context.Context, area *storage.Area, repo *providers.GitRepo) error {
	_, err := pr.git.Add(ctx, area, repo)
	if err != nil {
		return fmt.Errorf("could not add file: %w", err)
	}

	err = pr.git.Commit(ctx, repo)
	if err != nil {
		return fmt.Errorf("could not get diff: %w", err)
	}

	err = pr.git.Push(ctx, repo)
	if err != nil {
		return fmt.Errorf("could not push to repo: %w", err)
	}
	return nil
}
