package actions

import "context"

type Predicate func(ctx context.Context, path string) (bool, error)
type Action func(ctx context.Context, path string) error
