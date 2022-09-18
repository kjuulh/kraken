package serverdeps

import (
	actionc "git.front.kjuulh.io/kjuulh/kraken/internal/actions"
	"git.front.kjuulh.io/kjuulh/kraken/internal/gitproviders"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/actions"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/providers"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/signer"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/storage"
	"go.uber.org/zap"
)

type ServerDeps struct {
	logger *zap.Logger

	storageConfig *storage.StorageConfig
	gitCfg        *providers.GitConfig

	openPGP *signer.OpenPGP
}

func NewServerDeps(logger *zap.Logger) *ServerDeps {
	deps := &ServerDeps{
		logger: logger.With(zap.Namespace("serverdeps")),
	}

	if storageCfg, err := storage.NewDefaultStorageConfig(); err != nil {
		panic(err)
	} else {
		deps.storageConfig = storageCfg
	}

	deps.gitCfg = &providers.GitConfig{
		AuthOption:            providers.GIT_AUTH_SSH,
		User:                  "git",
		Password:              "",
		AccessToken:           "",
		SshPublicKeyFilePath:  "/Users/kah/.ssh/id_ed25519",
		SshPrivateKeyPassword: "",
	}

	openPGPConfig := &signer.OpenPgpConfig{
		PrivateKeyFilePath: "./example/testkey.private.pgp",
		PrivateKeyPassword: "somepassword",
		PrivateKeyIdentity: "kraken@kasperhermansen.com",
	}
	deps.openPGP = signer.NewOpenPGP(logger.With(zap.Namespace("openpgp")), openPGPConfig)

	return deps
}

func (deps *ServerDeps) GetStorageService() *storage.Service {
	return storage.NewService(deps.logger.With(zap.Namespace("storage")), deps.storageConfig)
}

func (deps *ServerDeps) GetGitProvider() *providers.Git {
	return providers.NewGit(deps.logger.With(zap.Namespace("gitProvider")), deps.gitCfg, deps.openPGP)
}

func (deps *ServerDeps) GetAction() *actions.Action {
	return actions.NewAction(deps.logger.With(zap.Namespace("action")))
}

func (deps *ServerDeps) GetActionCreator() *actionc.ActionCreator {
	return actionc.NewActionCreator(deps.logger.With(zap.Namespace("action")), deps)
}

func (deps *ServerDeps) GetGitea() *gitproviders.Gitea {
	return gitproviders.NewGitea(deps.logger.With(zap.Namespace("gitea")))
}

func (deps *ServerDeps) GetOpenPGP() *signer.OpenPGP {
	return deps.openPGP
}
