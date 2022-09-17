package commands

import (
	"context"
	"fmt"
	"strings"
	"sync"
	"time"

	"git.front.kjuulh.io/kjuulh/kraken/internal/actions"
	"git.front.kjuulh.io/kjuulh/kraken/internal/gitproviders"
	"git.front.kjuulh.io/kjuulh/kraken/internal/schema"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/providers"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/storage"
	giturls "github.com/whilp/git-urls"
	"go.uber.org/zap"
)

type (
	ProcessRepos struct {
		logger        *zap.Logger
		storage       *storage.Service
		git           *providers.Git
		actionCreator *actions.ActionCreator
		gitea         *gitproviders.Gitea
	}

	ProcessReposDeps interface {
		GetStorageService() *storage.Service
		GetGitProvider() *providers.Git
		GetActionCreator() *actions.ActionCreator
		GetGitea() *gitproviders.Gitea
	}
)

func NewProcessRepos(logger *zap.Logger, deps ProcessReposDeps) *ProcessRepos {
	return &ProcessRepos{
		logger:        logger,
		storage:       deps.GetStorageService(),
		git:           deps.GetGitProvider(),
		actionCreator: deps.GetActionCreator(),
		gitea:         deps.GetGitea(),
	}
}

func (pr *ProcessRepos) Process(ctx context.Context, repository string, branch string, actionPath string) error {
	action, err := pr.actionCreator.Prepare(ctx, &actions.ActionCreatorOps{
		RepositoryUrl: repository,
		Branch:        branch,
		Path:          actionPath,
	})
	if err != nil {
		return err
	}

	repositoryUrls, err := pr.getRepoUrls(ctx, action.Schema)
	if err != nil {
		return err
	}

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
			}
		}(ctx, repoUrl)
	}

	wg.Wait()
	pr.logger.Debug("finished processing all repos", zap.Strings("repos", repositoryUrls))

	return nil
}

func (pr *ProcessRepos) getRepoUrls(ctx context.Context, schema *schema.KrakenSchema) ([]string, error) {
	repoUrls := make([]string, 0)

	repoUrls = append(repoUrls, schema.Select.Repositories...)

	for _, provider := range schema.Select.Providers {
		repos, err := pr.gitea.ListRepositoriesForOrganization(ctx, provider.Gitea, provider.Organisation)
		if err != nil {
			return nil, err
		}

		repoUrls = append(repoUrls, repos...)
	}

	return repoUrls, nil
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

	err = pr.commit(ctx, area, repo, repoUrl)
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
		return nil, err
	}

	err = pr.git.CreateBranch(ctx, repo)
	if err != nil {
		return nil, err
	}

	return repo, nil
}

func (pr *ProcessRepos) commit(ctx context.Context, area *storage.Area, repo *providers.GitRepo, repoUrl string) error {
	wt, err := pr.git.Add(ctx, area, repo)
	if err != nil {
		return fmt.Errorf("could not add file: %w", err)
	}

	err = pr.git.Commit(ctx, repo)
	if err != nil {
		return fmt.Errorf("could not get diff: %w", err)
	}

	dryrun := false
	if !dryrun {
		status, err := wt.Status()
		if err != nil {
			return err
		}

		if status.IsClean() {
			pr.logger.Info("Returning early, as no modifications are detected")
			return nil
		}

		err = pr.git.Push(ctx, repo)
		if err != nil {
			return fmt.Errorf("could not push to repo: %w", err)
		}

		url, err := giturls.Parse(repoUrl)
		if err != nil {
			return err
		}

		head, err := repo.GetHEAD()
		if err != nil {
			return err
		}

		path := strings.Split(url.Path, "/")
		pr.logger.Debug("path string", zap.Strings("paths", path), zap.String("HEAD", head))

		org := path[0]
		repoName := path[1]
		semanticName, _, ok := strings.Cut(repoName, ".")
		if !ok {
			semanticName = repoName
		}

		originHead, err := pr.git.GetOriginHEADForRepo(ctx, repo)
		if err != nil {
			return err
		}

		err = pr.gitea.CreatePr(ctx, fmt.Sprintf("%s://%s", "https", url.Host), org, semanticName, head, originHead, "kraken-apply")
		if err != nil {
			return err
		}
	}
	return nil
}
