package builders

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"errors"
	"fmt"
	"os"
	"os/exec"

	"go.uber.org/zap"
	"go.uber.org/zap/zapio"
)

type DockerBuild struct {
	logger *zap.Logger
}

func NewDockerBuild(logger *zap.Logger) *DockerBuild {
	return &DockerBuild{logger: logger}
}

type DockerRunCommand func(ctx context.Context, victimPath string) error

func (g *DockerBuild) Build(ctx context.Context, modulePath, entryPath string) (DockerRunCommand, error) {
	g.logger.Debug("Building docker image", zap.String("actiondir", modulePath), zap.String("entry", entryPath))

	if _, err := os.Stat(fmt.Sprintf("%s/%s", modulePath, entryPath)); os.IsNotExist(err) {
		return nil, errors.New("could not find entry")
	}

	b := make([]byte, 20)
	_, err := rand.Reader.Read(b)
	if err != nil {
		return nil, err
	}
	tag := hex.EncodeToString(b)
	buildDockerCmd := fmt.Sprintf("(cd %s; docker build -f %s --tag kraken/%s .)", modulePath, entryPath, tag)
	g.logger.Debug("Running command", zap.String("command", buildDockerCmd))

	cmd := exec.CommandContext(
		ctx,
		"/bin/bash",
		"-c",
		buildDockerCmd,
	)

	debugwriter := &zapio.Writer{
		Log:   g.logger,
		Level: zap.DebugLevel,
	}
	defer debugwriter.Close()

	cmd.Stdout = debugwriter
	cmd.Stderr = debugwriter
	err = cmd.Start()
	if err != nil {
		return nil, err
	}

	err = cmd.Wait()
	if err != nil {
		return nil, err
	}

	g.logger.Debug("Docker image built!")

	return func(ctx context.Context, victimPath string) error {
		g.logger.Debug("Executing script", zap.String("victim", victimPath))

		cmd := exec.CommandContext(
			ctx,
			"/bin/bash",
			"-c",
			fmt.Sprintf("docker run --rm -v %s/:/src/work/ kraken/%s", victimPath, tag),
		)

		runDockerWriter := &zapio.Writer{
			Log:   g.logger,
			Level: zap.DebugLevel,
		}
		defer runDockerWriter.Close()

		cmd.Stdout = runDockerWriter
		cmd.Stderr = runDockerWriter

		err = cmd.Start()
		if err != nil {
			return err
		}

		return cmd.Wait()
	}, nil
}
