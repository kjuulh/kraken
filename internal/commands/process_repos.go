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
	// Clone repos
	wg := sync.WaitGroup{}
	wg.Add(len(repositoryUrls))
	errChan := make(chan error, 1)

	for _, repoUrl := range repositoryUrls {
		go func(ctx context.Context, repoUrl string) {
			defer func() {
				wg.Done()
			}()
			pr.logger.Debug("Creating area", zap.String("repoUrl", repoUrl))
			area, err := pr.storage.CreateArea(ctx)
			if err != nil {
				pr.logger.Error("failed to allocate area", zap.Error(err))
				errChan <- err
				return
			}

			defer func(ctx context.Context) {
				pr.logger.Debug("Removing area", zap.String("path", area.Path), zap.String("repoUrl", repoUrl))
				err = pr.storage.RemoveArea(ctx, area)
				if err != nil {
					errChan <- err
					return
				}

			}(ctx)

			pr.logger.Debug("Cloning repo", zap.String("path", area.Path), zap.String("repoUrl", repoUrl))
			cloneCtx, _ := context.WithTimeout(ctx, time.Second*5)
			repo, err := pr.git.Clone(cloneCtx, area, repoUrl)
			if err != nil {
				pr.logger.Error("could not clone repo", zap.Error(err))
				errChan <- err
				return
			}

			err = pr.git.CreateBranch(ctx, repo)
			if err != nil {
				pr.logger.Error("could not create branch", zap.Error(err))
				errChan <- err
				return
			}

			err = pr.action.Run(
				ctx,
				area,
				func(_ context.Context, area *storage.Area) (bool, error) {
					pr.logger.Debug("checking predicate", zap.String("area", area.Path))
					contains := false
					filepath.WalkDir(area.Path, func(path string, d fs.DirEntry, err error) error {
						if d.Name() == "roadmap.md" {
							contains = true
						}
						return nil
					})
					return contains, nil
				},
				func(_ context.Context, area *storage.Area) error {
					pr.logger.Debug("running action", zap.String("area", area.Path))
					readme := path.Join(area.Path, "README.md")
					file, err := os.Create(readme)
					if err != nil {
						return fmt.Errorf("could not create readme: %w", err)
					}
					_, err = file.WriteString("# Readme")
					if err != nil {
						return fmt.Errorf("could not write readme: %w", err)
					}

					_, err = pr.git.Add(ctx, area, repo)
					if err != nil {
						return fmt.Errorf("could not add file: %w", err)
					}

					err = pr.git.Commit(ctx, repo)
					if err != nil {
						return fmt.Errorf("could not get diff: %w", err)
					}

					return nil
				}, false)
			if err != nil {
				pr.logger.Error("could not run action", zap.Error(err))
				errChan <- err
				return
			}

			err = pr.git.Push(ctx, repo)
			if err != nil {
				pr.logger.Error("could not push to repo", zap.Error(err))
				errChan <- err
				return
			}

			pr.logger.Debug("processing done", zap.String("path", area.Path), zap.String("repoUrl", repoUrl))
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
