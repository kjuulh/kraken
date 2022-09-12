//go:build integration
// +build integration

package integrationtest_test

import (
	"os"
	"testing"

	"git.front.kjuulh.io/kjuulh/kraken/internal/server"
	"go.uber.org/zap"
)

func MainTest(t *testing.M) {
	logger, err := zap.NewDevelopment()
	if err != nil {
		panic(err)
	}
	err = server.Start(logger)
	if err != nil {
		panic(err)
	}

	os.Exit(t.Run())
}
