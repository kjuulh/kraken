package providers

import (
	"context"
	"fmt"
	"time"

	"git.front.kjuulh.io/kjuulh/kraken/internal/services/signer"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/storage"
	"github.com/go-git/go-git/v5"
	"github.com/go-git/go-git/v5/config"
	"github.com/go-git/go-git/v5/plumbing"
	"github.com/go-git/go-git/v5/plumbing/object"
	"github.com/go-git/go-git/v5/plumbing/transport"
	"github.com/go-git/go-git/v5/plumbing/transport/http"
	"github.com/go-git/go-git/v5/plumbing/transport/ssh"
	"go.uber.org/zap"
	"go.uber.org/zap/zapio"
)

// Git is a native git provider, it can clone, pull
// , push and as in abstraction on native git operations
type Git struct {
	logger    *zap.Logger
	gitConfig *GitConfig
	openPGP   *signer.OpenPGP
}

type GitRepo struct {
	repo *git.Repository
}

func (gr *GitRepo) GetHEAD() (string, error) {
	head, err := gr.repo.Head()
	if err != nil {
		return "", err
	}

	return head.Name().Short(), nil
}

type GitAuth string

const (
	GIT_AUTH_SSH               GitAuth = "ssh"
	GIT_AUTH_USERNAME_PASSWORD GitAuth = "username_password"
	GIT_AUTH_ACCESS_TOKEN      GitAuth = "access_token"
	GIT_AUTH_ANONYMOUS         GitAuth = "anonymous"
	GIT_AUTH_SSH_AGENT         GitAuth = "ssh_agent"
)

type GitConfig struct {
	AuthOption            GitAuth
	User                  string
	Password              string
	AccessToken           string
	SshPublicKeyFilePath  string
	SshPrivateKeyPassword string
}

func NewGit(logger *zap.Logger, gitConfig *GitConfig, openPGP *signer.OpenPGP) *Git {
	return &Git{logger: logger, gitConfig: gitConfig, openPGP: openPGP}
}

func (g *Git) GetOriginHEADForRepo(ctx context.Context, gitRepo *GitRepo) (string, error) {
	remote, err := gitRepo.repo.Remote("origin")
	if err != nil {
		return "", err
	}

	auth, err := g.GetAuth()
	if err != nil {
		return "", err
	}

	refs, err := remote.ListContext(ctx, &git.ListOptions{
		Auth: auth,
	})
	if err != nil {
		return "", err
	}

	headRef := ""
	for _, ref := range refs {
		if !ref.Name().IsBranch() {
			headRef = ref.Target().Short()
		}
	}

	return headRef, nil
}

func (g *Git) CloneBranch(ctx context.Context, storageArea *storage.Area, repoUrl string, branch string) (*GitRepo, error) {
	g.logger.Debug(
		"cloning repository",
		zap.String("repoUrl", repoUrl),
		zap.String("path", storageArea.Path),
	)

	auth, err := g.GetAuth()
	if err != nil {
		return nil, err
	}

	cloneOptions := git.CloneOptions{
		URL:               repoUrl,
		Auth:              auth,
		RemoteName:        "origin",
		ReferenceName:     plumbing.NewBranchReferenceName(branch),
		SingleBranch:      true,
		NoCheckout:        false,
		Depth:             1,
		RecurseSubmodules: 1,
		Progress:          g.getProgressWriter(),
		Tags:              0,
		InsecureSkipTLS:   false,
		CABundle:          []byte{},
	}

	repo, err := git.PlainCloneContext(ctx, storageArea.Path, false, &cloneOptions)
	if err != nil {
		return nil, err
	}

	g.logger.Debug("done cloning repo")

	return &GitRepo{repo: repo}, nil
}

func (g *Git) Clone(ctx context.Context, storageArea *storage.Area, repoUrl string) (*GitRepo, error) {
	g.logger.Debug(
		"cloning repository",
		zap.String("repoUrl", repoUrl),
		zap.String("path", storageArea.Path),
	)

	auth, err := g.GetAuth()
	if err != nil {
		return nil, err
	}

	cloneOptions := git.CloneOptions{
		URL:               repoUrl,
		Auth:              auth,
		RemoteName:        "origin",
		ReferenceName:     "refs/heads/main",
		SingleBranch:      true,
		NoCheckout:        false,
		Depth:             1,
		RecurseSubmodules: 1,
		Progress:          g.getProgressWriter(),
		Tags:              0,
		InsecureSkipTLS:   false,
		CABundle:          []byte{},
	}

	repo, err := git.PlainCloneContext(ctx, storageArea.Path, false, &cloneOptions)
	if err != nil {
		return nil, err
	}

	g.logger.Debug("done cloning repo")

	return &GitRepo{repo: repo}, nil
}

func (g *Git) getProgressWriter() *zapio.Writer {
	return &zapio.Writer{
		Log:   g.logger.With(zap.String("process", "go-git")),
		Level: zap.DebugLevel,
	}
}

func (g *Git) Add(ctx context.Context, storageArea *storage.Area, gitRepo *GitRepo) (*git.Worktree, error) {
	worktree, err := gitRepo.repo.Worktree()
	if err != nil {
		return nil, err
	}

	err = worktree.AddWithOptions(&git.AddOptions{
		All: true,
	})
	if err != nil {
		return nil, err
	}

	status, err := worktree.Status()
	if err != nil {
		return nil, err
	}

	g.logger.Debug("git status", zap.String("status", status.String()))

	return worktree, nil
}

func (g *Git) CreateBranch(ctx context.Context, gitRepo *GitRepo) error {
	worktree, err := gitRepo.repo.Worktree()
	if err != nil {
		return err
	}

	refSpec := plumbing.NewBranchReferenceName("kraken-apply")
	err = gitRepo.repo.CreateBranch(&config.Branch{
		Name:   "kraken-apply",
		Remote: "origin",
		Merge:  refSpec,
		Rebase: "",
	})
	if err != nil {
		return fmt.Errorf("could not create branch: %w", err)
	}

	err = worktree.Checkout(&git.CheckoutOptions{
		Branch: plumbing.ReferenceName(refSpec.String()),
		Create: true,
		Force:  false,
		Keep:   false,
	})
	if err != nil {
		return fmt.Errorf("could not checkout branch: %w", err)
	}

	remoteRef := plumbing.NewRemoteReferenceName("origin", "kraken-apply")
	ref := plumbing.NewSymbolicReference(refSpec, remoteRef)
	err = gitRepo.repo.Storer.SetReference(ref)
	if err != nil {
		return fmt.Errorf("could not set reference: %w", err)
	}

	auth, err := g.GetAuth()
	if err != nil {
		return err
	}

	err = worktree.PullContext(ctx, &git.PullOptions{
		RemoteName:        "origin",
		ReferenceName:     "refs/heads/main",
		SingleBranch:      true,
		Depth:             1,
		Auth:              auth,
		RecurseSubmodules: 1,
		Progress:          g.getProgressWriter(),
		Force:             true,
		InsecureSkipTLS:   false,
		CABundle:          []byte{},
	})
	if err != nil {
		return fmt.Errorf("could not pull from origin: %w", err)
	}

	g.logger.Debug("done creating branches")

	return nil
}

func (g *Git) Commit(ctx context.Context, gitRepo *GitRepo) error {
	worktree, err := gitRepo.repo.Worktree()
	if err != nil {
		return err
	}

	_, err = worktree.Commit("some-commit", &git.CommitOptions{
		All:       true,
		Author:    &object.Signature{Name: "kraken", Email: "kraken@kasperhermansen.com", When: time.Now()},
		Committer: &object.Signature{Name: "kraken", Email: "kraken@kasperhermansen.com", When: time.Now()},
		SignKey:   g.openPGP.SigningKey,
	})
	if err != nil {
		return err
	}

	g.logger.Debug("done commiting objects")

	return nil
}

func (g *Git) Push(ctx context.Context, gitRepo *GitRepo) error {
	auth, err := g.GetAuth()
	if err != nil {
		return err
	}

	err = gitRepo.repo.PushContext(ctx, &git.PushOptions{
		RemoteName:        "origin",
		RefSpecs:          []config.RefSpec{},
		Auth:              auth,
		Progress:          g.getProgressWriter(),
		Prune:             false,
		Force:             false,
		InsecureSkipTLS:   false,
		CABundle:          []byte{},
		RequireRemoteRefs: []config.RefSpec{},
	})

	if err != nil {
		return err
	}

	g.logger.Debug("done pushing branch")

	return nil
}

func (g *Git) GetAuth() (transport.AuthMethod, error) {
	switch g.gitConfig.AuthOption {
	case GIT_AUTH_SSH:
		sshKey, err := ssh.NewPublicKeysFromFile(
			g.gitConfig.User,
			g.gitConfig.SshPublicKeyFilePath,
			g.gitConfig.SshPrivateKeyPassword,
		)
		if err != nil {
			return nil, err
		}
		return sshKey, nil
	case GIT_AUTH_USERNAME_PASSWORD:
		return &http.BasicAuth{
			Username: g.gitConfig.User,
			Password: g.gitConfig.Password,
		}, nil
	case GIT_AUTH_ACCESS_TOKEN:
		return &http.BasicAuth{
			Username: "required-username",
			Password: g.gitConfig.AccessToken,
		}, nil
	case GIT_AUTH_ANONYMOUS:
		return nil, nil
	case GIT_AUTH_SSH_AGENT:
		return ssh.NewSSHAgentAuth(g.gitConfig.User)
	default:
		return nil, nil
	}
}
