package actions

import (
	"context"
	"errors"

	"git.front.kjuulh.io/kjuulh/kraken/internal/actions/builders"
	"git.front.kjuulh.io/kjuulh/kraken/internal/actions/querier"
	"git.front.kjuulh.io/kjuulh/kraken/internal/schema"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/storage"
	"go.uber.org/zap"
)

type Action struct {
	Schema     *schema.KrakenSchema
	SchemaPath string
}

func (a *Action) Execute(ctx context.Context, area *storage.Area) error {
	for _, action := range a.Schema.Actions {
		switch action.Type {
		case "go":
			exe, err := builders.NewGo(zap.L()).Build(ctx, a.SchemaPath, action.Entry)
			if err != nil {
				return err
			}
			err = exe(ctx, area.Path)
			if err != nil {
				return err
			}

			zap.L().Debug("Execution done")

		case "docker-build":
			zap.L().Debug("Building docker-build")
			runCmd, err := builders.NewDockerBuild(zap.L()).Build(ctx, a.SchemaPath, action.Entry)
			if err != nil {
				return err
			}
			err = runCmd(ctx, area.Path)
			if err != nil {
				return err
			}
			return nil

		default:
			return errors.New("could not determine action type")
		}
	}

	return nil
}

func (a *Action) Query(ctx context.Context, area *storage.Area) ([]string, bool, error) {
	for _, query := range a.Schema.Queries {
		switch query.Type {
		case "grep":
			exe, err := querier.NewRipGrep(zap.L()).Build(ctx, a.SchemaPath, query.Query)
			if err != nil {
				return nil, false, err
			}
			output, found, err := exe(ctx, area.Path)
			if err != nil {
				return nil, false, err
			}

			zap.L().Debug("Execution done")

			return output, found, nil

		default:
			return nil, false, errors.New("could not determine query type")
		}
	}

	return nil, false, nil
}
