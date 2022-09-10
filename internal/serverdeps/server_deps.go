package serverdeps

import "go.uber.org/zap"

type ServerDeps struct {
	logger *zap.Logger
}

func NewServerDeps(logger *zap.Logger) *ServerDeps {
	return &ServerDeps{
		logger: logger.With(zap.String("app", "serverdeps")),
	}
}
