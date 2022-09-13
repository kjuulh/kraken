package actions

import (
	"context"

	"git.front.kjuulh.io/kjuulh/kraken/internal/schema"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/storage"
)

type Action struct {
	Schema *schema.KrakenSchema
}

func (a *Action) Execute(ctx context.Context, area *storage.Area) error {
	return nil
}
