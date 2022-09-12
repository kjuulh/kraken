package server

import (
	"context"
	"errors"
	"net/http"
	"time"

	"git.front.kjuulh.io/kjuulh/curre"
	"git.front.kjuulh.io/kjuulh/kraken/internal/api"
	"git.front.kjuulh.io/kjuulh/kraken/internal/serverdeps"
	ginzap "github.com/gin-contrib/zap"
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
)

func NewGinHttpServer(logger *zap.Logger, deps *serverdeps.ServerDeps) curre.Component {
	var app *gin.Engine
	var server *http.Server

	return curre.NewFunctionalComponent(&curre.FunctionalComponent{
		InitFunc: func(_ *curre.FunctionalComponent, _ context.Context) error {
			app = gin.New()
			app.UseH2C = true
			app.Use(ginzap.Ginzap(logger, time.RFC3339, true))
			app.Use(ginzap.RecoveryWithZap(logger, true))

			api.BuildApi(logger, app, deps)

			server = &http.Server{
				Addr:    "127.0.0.1:3000",
				Handler: app,
			}

			return nil
		},
		StartFunc: func(_ *curre.FunctionalComponent, _ context.Context) error {
			if server != nil {
				err := server.ListenAndServe()
				if err != nil && !errors.Is(err, http.ErrServerClosed) {
					return err
				}
			}
			return nil
		},
		StopFunc: func(_ *curre.FunctionalComponent, ctx context.Context) error {
			ctx, _ = context.WithTimeout(ctx, time.Second*10)
			if server != nil {
				server.Shutdown(ctx)
			}
			return nil
		},
	})
}
