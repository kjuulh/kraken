package signer

import (
	"context"
	"errors"
	"os"
	"strings"

	"git.front.kjuulh.io/kjuulh/curre"
	"github.com/ProtonMail/go-crypto/openpgp"
	"go.uber.org/zap"
)

type OpenPGP struct {
	logger     *zap.Logger
	SigningKey *openpgp.Entity
	config     *OpenPgpConfig
}

type OpenPgpConfig struct {
	PrivateKeyFilePath string
	PrivateKeyPassword string
	PrivateKeyIdentity string
}

func NewOpenPGP(logger *zap.Logger, config *OpenPgpConfig) *OpenPGP {
	return &OpenPGP{
		logger: logger,
		config: config,
	}
}

func NewOpenPGPApp(openPGP *OpenPGP) curre.Component {
	return curre.NewFunctionalComponent(&curre.FunctionalComponent{
		InitFunc: func(_ *curre.FunctionalComponent, ctx context.Context) error {
			keyring, err := buildKeyring(ctx, openPGP)
			if err != nil {
				openPGP.logger.Panic("could not build keyring", zap.Error(err))
				return err
			}

			openPGP.SigningKey = keyring

			return nil
		},
		StartFunc: func(fc *curre.FunctionalComponent, ctx context.Context) error {
			return nil
		},
		StopFunc: func(fc *curre.FunctionalComponent, ctx context.Context) error {
			return nil
		},
	})
}

func buildKeyring(_ context.Context, openPGP *OpenPGP) (*openpgp.Entity, error) {
	content, err := os.ReadFile(openPGP.config.PrivateKeyFilePath)
	if err != nil {
		return nil, err
	}
	reader := strings.NewReader(string(content))

	es, err := openpgp.ReadArmoredKeyRing(reader)
	if err != nil {
		return nil, err
	}

	for _, key := range es {
		for k := range key.Identities {
			if strings.Contains(k, openPGP.config.PrivateKeyIdentity) {
				err = key.PrivateKey.Decrypt([]byte(openPGP.config.PrivateKeyPassword))
				if err != nil {
					return nil, err
				}
				return key, nil
			}
		}
	}

	return nil, errors.New("could not find key matching identity")

}
