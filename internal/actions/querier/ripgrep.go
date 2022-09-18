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

type RipGrepCommand func(ctx context.Context, victimPath string) ([]string, bool, error)

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

	return func(ctx context.Context, victimPath string) ([]string, bool, error) {
		g.logger.Debug("Executing script", zap.String("victim", victimPath))

		runRipGrepCmd := fmt.Sprintf("docker run --rm -v %s/:/data:ro mbologna/docker-ripgrep rg -i '%s' || true", victimPath, query)

		g.logger.Debug("Execute ripgrep query", zap.String("command", runRipGrepCmd))

		cmd := exec.CommandContext(
			ctx,
			"/bin/bash",
			"-c",
			runRipGrepCmd,
		)

		runDockerWriter := &zapio.Writer{
			Log:   g.logger,
			Level: zap.DebugLevel,
		}
		defer runDockerWriter.Close()

		builder := &strings.Builder{}
		combinedWriter := io.MultiWriter(runDockerWriter, builder)

		cmd.Stdout = combinedWriter
		cmd.Stderr = combinedWriter

		err = cmd.Start()
		if err != nil {
			return nil, false, err
		}

		err = cmd.Wait()
		if err != nil {
			return nil, false, err
		}

		contents := strings.Split(builder.String(), "\n")
		validatedOutput := make([]string, 0)

		for _, c := range contents {
			if !strings.Contains(c, "WARNING:") {
				validatedOutput = append(validatedOutput, c)
			}
		}

		found := len(validatedOutput) > 0

		return validatedOutput, found, nil
	}, nil
}
