//go:build integration
// +build integration

package integrationtest_test

import (
	"context"
	"os"
	"path"
	"testing"

	"git.front.kjuulh.io/kjuulh/kraken/internal/services/storage"
	"github.com/stretchr/testify/require"
)

func TestInitializeStorage(t *testing.T) {
	t.Parallel()
	storage, cfg := prepareService(t)
	err := storage.InitializeStorage(context.Background())
	require.NoError(t, err)

	if _, err := os.Stat(path.Join(cfg.Path, "storage")); os.IsNotExist(err) {
		require.NoError(t, err, "could not create storage directory")
	}
}

func TestCleanupStorage(t *testing.T) {
	t.Parallel()
	storage, _ := prepareService(t)
	err := storage.InitializeStorage(context.Background())
	require.NoError(t, err)

	err = storage.CleanupStorage(context.Background())
	require.NoError(t, err)
}

func TestCreateArea(t *testing.T) {
	t.Parallel()
	storage, cfg := prepareService(t)
	err := storage.InitializeStorage(context.Background())
	require.NoError(t, err)

	area, err := storage.CreateArea(context.Background())
	require.NoError(t, err)
	require.NotNil(t, area)
	require.NotEmpty(t, area.Path)
	require.Contains(t, area.Path, cfg.Path)
}

func TestRemoveArea(t *testing.T) {
	t.Parallel()
	storage, _ := prepareService(t)
	err := storage.InitializeStorage(context.Background())
	require.NoError(t, err)
	area, err := storage.CreateArea(context.Background())
	require.NoError(t, err)

	err = storage.RemoveArea(context.Background(), area)
	require.NoError(t, err)

	if _, err := os.Stat(area.Path); os.IsNotExist(err) {
		require.Error(t, err, "directory could not be removed")
		return
	}
	t.Fatal("directory could not be removed")
}

func prepareService(t *testing.T) (*storage.Service, *storage.StorageConfig) {
	cfg := &storage.StorageConfig{
		Path: t.TempDir(),
	}

	return storage.NewService(cfg), cfg
}
