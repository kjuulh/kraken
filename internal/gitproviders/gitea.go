package gitproviders

import (
	"context"
	"fmt"
	"sync"

	"code.gitea.io/sdk/gitea"
	"go.uber.org/zap"
)

type Gitea struct {
	logger       *zap.Logger
	giteamu      sync.Mutex
	giteaClients map[string]*gitea.Client
}

func NewGitea(logger *zap.Logger) *Gitea {
	return &Gitea{
		logger:       logger,
		giteamu:      sync.Mutex{},
		giteaClients: make(map[string]*gitea.Client, 0),
	}
}

func (g *Gitea) ListRepositoriesForOrganization(
	ctx context.Context,
	server string,
	organization string,
) ([]string, error) {
	client, err := g.getOrCreateClient(ctx, server)
	if err != nil {
		return nil, err
	}

	g.logger.Debug("Listing repos for gitea", zap.String("server", server))
	repos, resp, err := client.ListOrgRepos(organization, gitea.ListOrgReposOptions{
		ListOptions: gitea.ListOptions{
			Page:     0,
			PageSize: 20,
		},
	})
	if err != nil {
		return nil, fmt.Errorf("could not list repos: %w", err)
	}

	if resp.StatusCode >= 300 {
		return nil, fmt.Errorf("gitea responded with a non 200 status code (gitea response: %s)", resp.Status)
	}

	repoUrls := make([]string, len(repos))
	for i, repo := range repos {
		repoUrls[i] = repo.SSHURL
	}

	return repoUrls, err
}

func (g *Gitea) CreatePr(
	ctx context.Context,
	server string,
	organization string,
	repository string,
	head string,
	base string,
	actionName string,
) error {
	client, err := g.getOrCreateClient(ctx, server)
	if err != nil {
		return err
	}

	prs, _, err := client.ListRepoPullRequests(organization, repository, gitea.ListPullRequestsOptions{
		ListOptions: gitea.ListOptions{
			Page:     0,
			PageSize: 30,
		},
		State:     gitea.StateOpen,
		Sort:      "recentupdate",
		Milestone: 0,
	})
	if err != nil {
		return fmt.Errorf(
			"could not list repos, needed because we need to check for conflicts. Original error: %w",
			err,
		)
	}
	for _, pr := range prs {
		if pr.Head.Name == head {
			g.logger.Info(
				"returning early from creating pull-request, as it already exists.",
				zap.String("repository", repository),
				zap.String("pull-request", pr.URL),
			)
			return nil
		}
	}

	pr, _, err := client.CreatePullRequest(organization, repository, gitea.CreatePullRequestOption{
		Head:  head,
		Base:  base,
		Title: actionName,
	})
	if err != nil {
		return err
	}

	g.logger.Debug(
		"Created pr",
		zap.String("repository", repository),
		zap.String("branch", head),
		zap.String("pull-request", pr.URL),
	)

	return nil
}

func (g *Gitea) getOrCreateClient(ctx context.Context, server string) (*gitea.Client, error) {
	g.giteamu.Lock()
	defer g.giteamu.Unlock()
	client, ok := g.giteaClients[server]
	if !ok || client == nil {
		c, err := gitea.NewClient(server)
		c.SetBasicAuth("kjuulh", "c0bd801cc9a7f2ed559ea45d603afc92f5443f19")
		if err != nil {
			return nil, err
		}
		g.giteaClients[server] = c
		return c, nil
	}

	return client, nil
}
