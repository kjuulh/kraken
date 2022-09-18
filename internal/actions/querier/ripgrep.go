package querier

import (
	"context"
	"fmt"
	"io"
	"os/exec"
	"strings"

	"go.uber.org/zap"
	"go.uber.org/zap/zapio"
)

type RipGrep struct {
	logger *zap.Logger
}

func NewRipGrep(logger *zap.Logger) *RipGrep {
	return &RipGrep{logger: logger}
}

type RipGrepCommand func(ctx context.Context, victimPath string) ([]string, error)

func (g *RipGrep) Build(ctx context.Context, modulePath, query string) (RipGrepCommand, error) {
	g.logger.Debug("Pulling docker image", zap.String("actiondir", modulePath), zap.String("query", query))

	pullDockerImage := "docker pull mbologna/docker-ripgrep"
	g.logger.Debug("Running command", zap.String("command", pullDockerImage))

	cmd := exec.CommandContext(
		ctx,
		"/bin/bash",
		"-c",
		pullDockerImage,
	)

	debugwriter := &zapio.Writer{
		Log:   g.logger,
		Level: zap.DebugLevel,
	}
	defer debugwriter.Close()

	cmd.Stdout = debugwriter
	cmd.Stderr = debugwriter
	err := cmd.Start()
	if err != nil {
		return nil, err
	}

	err = cmd.Wait()
	if err != nil {
		return nil, err
	}

	g.logger.Debug("Docker image pulled")

	return func(ctx context.Context, victimPath string) ([]string, error) {
		g.logger.Debug("Executing script", zap.String("victim", victimPath))

		cmd := exec.CommandContext(
			ctx,
			"/bin/bash",
			"-c",
			fmt.Sprintf("docker run --rm -v %s/:/data mbologna/docker-ripgrep rg -i %s", victimPath, query),
		)

		runDockerWriter := &zapio.Writer{
			Log:   g.logger,
			Level: zap.DebugLevel,
		}
		defer runDockerWriter.Close()

		builder := &strings.Builder{}

		combinedWriter := io.MultiWriter(runDockerWriter, builder)

		cmd.Stdout = combinedWriter
		cmd.Stderr = runDockerWriter

		err = cmd.Start()
		if err != nil {
			return nil, err
		}

		err = cmd.Wait()

		if err != nil {
			return nil, err
		}

		contents := strings.Split(builder.String(), "\n")

		return contents, nil
	}, nil
}
