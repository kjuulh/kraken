package commands

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path"
	"path/filepath"
	"sync"
	"time"

	"git.front.kjuulh.io/kjuulh/kraken/internal/services/actions"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/providers"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/storage"
	"go.uber.org/zap"
)

type (
	ProcessRepos struct {
		logger  *zap.Logger
		storage *storage.Service
		git     *providers.Git
		action  *actions.Action
	}

	ProcessReposDeps interface {
		GetStorageService() *storage.Service
		GetGitProvider() *providers.Git
		GetAction() *actions.Action
	}
)

func NewProcessRepos(logger *zap.Logger, deps ProcessReposDeps) *ProcessRepos {
	return &ProcessRepos{
		logger:  logger,
		storage: deps.GetStorageService(),
		git:     deps.GetGitProvider(),
		action:  deps.GetAction(),
	}
}

func (pr *ProcessRepos) Process(ctx context.Context, repositoryUrls []string) error {
	errChan := make(chan error, 1)

	wg := sync.WaitGroup{}
	wg.Add(len(repositoryUrls))

	for _, repoUrl := range repositoryUrls {
		go func(ctx context.Context, repoUrl string) {
			defer func() {
				wg.Done()
			}()
			pr.processRepo(ctx, repoUrl, errChan)
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

func (pr *ProcessRepos) processRepo(ctx context.Context, repoUrl string, errChan chan error) {
	cleanup, area, err := pr.prepareAction(ctx)
	defer func() {
		if cleanup != nil {
			cleanup(ctx, errChan)
		}
	}()
	if err != nil {
		errChan <- err
		return
	}

	repo, err := pr.clone(ctx, area, repoUrl)
	if err != nil {
		errChan <- err
		return
	}

	err = pr.action.Run(
		ctx,
		area,
		func(_ context.Context, area *storage.Area) (bool, error) {
			pr.logger.Debug("checking predicate", zap.String("area", area.Path))

			// TODO: Run predicate instead
			contains := false
			filepath.WalkDir(area.Path, func(_ string, d fs.DirEntry, _ error) error {
				if d.Name() == "roadmap.md" {
					contains = true
				}
				return nil
			})
			return contains, nil
		},
		func(_ context.Context, area *storage.Area) error {
			pr.logger.Debug("running action", zap.String("area", area.Path))

			// TODO: Run action instead
			readme := path.Join(area.Path, "README.md")
			file, err := os.Create(readme)
			if err != nil {
				return fmt.Errorf("could not create readme: %w", err)
			}

			_, err = file.WriteString("# Readme")
			if err != nil {
				return fmt.Errorf("could not write readme: %w", err)
			}

			err = pr.commit(ctx, area, repo)
			if err != nil {
				return err
			}

			return nil
		}, false)
	if err != nil {
		pr.logger.Error("could not run action", zap.Error(err))
		errChan <- err
		return
	}

	pr.logger.Debug("processing done", zap.String("path", area.Path), zap.String("repoUrl", repoUrl))
}

func (pr *ProcessRepos) prepareAction(
	ctx context.Context,
) (func(ctx context.Context, errChan chan error), *storage.Area, error) {
	pr.logger.Debug("Creating area")
	area, err := pr.storage.CreateArea(ctx)
	if err != nil {
		pr.logger.Error("failed to allocate area", zap.Error(err))
		return nil, nil, err
	}

	cleanupfunc := func(ctx context.Context, errChan chan error) {
		pr.logger.Debug("Removing area", zap.String("path", area.Path))
		err = pr.storage.RemoveArea(ctx, area)
		if err != nil {
			errChan <- err
			return
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
