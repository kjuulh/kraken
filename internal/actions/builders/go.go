package builders

import (
	"context"
	"errors"
	"fmt"
	"os"
	"os/exec"

	"go.uber.org/zap"
)

type Go struct {
	logger *zap.Logger
}

func NewGo(logger *zap.Logger) *Go {
	return &Go{logger: logger}
}

type GoExecutable func(ctx context.Context, victimPath string) error

func (g *Go) Build(ctx context.Context, modulePath, entryPath string) (GoExecutable, error) {
	g.logger.Debug("Building go binary", zap.String("actiondir", modulePath), zap.String("entry", entryPath))

	if _, err := os.Stat(fmt.Sprintf("%s/%s", modulePath, entryPath)); os.IsNotExist(err) {
		return nil, errors.New("could not find entry")
	}

	err := exec.CommandContext(
		ctx,
		"/bin/bash",
		"-c",
		fmt.Sprintf("(cd %s; go build -o main %s)", modulePath, entryPath),
	).Run()
	if err != nil {
		return nil, err
	}

	g.logger.Debug("Go binary built!")

	return func(ctx context.Context, victimPath string) error {
		g.logger.Debug("Executing script", zap.String("victim", victimPath))
		return exec.CommandContext(ctx, "/bin/bash", "-c", fmt.Sprintf("(cd %s; %s/main)", victimPath, modulePath)).Run()
	}, nil
}
