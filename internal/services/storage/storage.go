package storage

import (
	"errors"
	"os"
	"path"

	"go.uber.org/zap"
	"golang.org/x/net/context"
)

// The idea behind storage is that we have file dir, with a git repo.
// This file repo can now take certain actions

type StorageConfig struct {
	Path string
}

func NewDefaultStorageConfig() (*StorageConfig, error) {
	tempDir, err := os.MkdirTemp(os.TempDir(), "")
	if err != nil {
		return nil, err
	}
	return &StorageConfig{
		Path: path.Join(tempDir, "octopush"),
	}, nil
}

type Service struct {
	logger *zap.Logger
	cfg    *StorageConfig
}

func NewService(logger *zap.Logger, cfg *StorageConfig) *Service {
	return &Service{logger: logger, cfg: cfg}
}

func (s *Service) getStoragePath(ctx context.Context) string {
	return path.Join(s.cfg.Path, "storage")
}

func (s *Service) InitializeStorage(ctx context.Context) error {
	return os.MkdirAll(s.getStoragePath(ctx), 0755)
}

func (s *Service) CleanupStorage(ctx context.Context) error {
	doneRemovingChan := make(chan struct{}, 1)
	go func(ctx context.Context) {
		s.logger.Debug("Removing all temp storage")
		os.RemoveAll(s.getStoragePath(ctx))
		doneRemovingChan <- struct{}{}
	}(ctx)

	select {
	case <-ctx.Done():
		return errors.New("could not cleanup storage aborting")
	case <-doneRemovingChan:
		return nil
	}

	return nil
}

func (s *Service) CreateArea(ctx context.Context) (*Area, error) {
	dir, err := os.MkdirTemp(s.getStoragePath(ctx), "*")
	if err != nil {
		return nil, err
	}

	return &Area{
		Path: dir,
	}, nil
}

func (s *Service) RemoveArea(ctx context.Context, area *Area) error {
	return os.RemoveAll(area.Path)
}
