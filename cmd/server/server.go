package main

import (
	"os"

	"git.front.kjuulh.io/kjuulh/kraken/cmd/server/commands"
	"git.front.kjuulh.io/kjuulh/kraken/internal/logger"
	"go.uber.org/zap"
)

func main() {
	logger, err := logger.New()
	if err != nil {
		panic(err)
	}
	_ = logger.Sync()

	Execute(logger)
}

func Execute(logger *zap.Logger) {
	err := commands.CreateServerCmd(logger).Execute()
	if err != nil {
		logger.Error("execution failed", zap.Error(err))
		os.Exit(1)
	}
}
