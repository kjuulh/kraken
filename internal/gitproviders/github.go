package gitproviders

import (
	"context"
	"net/http"
	"sync"

	"github.com/google/go-github/github"
	"go.uber.org/zap"
)

type Github struct {
	logger       *zap.Logger
	githubmu     sync.Mutex
	githubClient map[string]*github.Client
}

func NewGitHub(logger *zap.Logger) *Github {
	return &Github{
		logger:   logger,
		githubmu: sync.Mutex{},
	}
}

func (g *Github) ListRepositoriesForOrganization(
	ctx context.Context,
	server string,
	organisation string,
) ([]string, error) {
	client, err := g.getOrCreateClient(ctx, server)

	repos, _, err := client.Repositories.ListByOrg(ctx, organisation, &github.RepositoryListByOrgOptions{
		ListOptions: github.ListOptions{
			Page:    0,
			PerPage: 0,
		},
	})

	if err != nil {
		return nil, err
	}

	repoUrls := make([]string, len(repos))
	for i, repo := range repos {
		repoUrls[i] = repo.GetCloneURL()
	}

	return repoUrls, nil
}

func (g *Github) getOrCreateClient(ctx context.Context, server string) (*github.Client, error) {
	g.githubmu.Lock()
	defer g.githubmu.Unlock()

	client, ok := g.githubClient[server]
	if !ok || client == nil {
		httpClient := &http.Client{}
		client := github.NewClient(httpClient)
		g.githubClient[server] = client
		return client, nil
	}

	return client, nil
}
