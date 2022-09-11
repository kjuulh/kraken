package server

import (
	"context"
	"errors"
	"net/http"

	"git.front.kjuulh.io/kjuulh/curre"
	"git.front.kjuulh.io/kjuulh/kraken/internal/serverdeps"
	"github.com/gin-gonic/gin"
)

func NewHttpServer(deps *serverdeps.ServerDeps) curre.Component {
	return curre.NewFunctionalComponent(&curre.FunctionalComponent{
		StartFunc: func(_ *curre.FunctionalComponent, _ context.Context) error {
			handler := http.NewServeMux()
			handler.HandleFunc(
				"/health/ready",
				func(w http.ResponseWriter, _ *http.Request) {
					w.Write([]byte("ready"))
					w.WriteHeader(http.StatusOK)
				})

			http.ListenAndServe("127.0.0.1:3000", handler)

			return nil
		},
	},
	)
}

func NewGinHttpServer(_ *serverdeps.ServerDeps) curre.Component {
	var app *gin.Engine
	var server *http.Server

	return curre.NewFunctionalComponent(&curre.FunctionalComponent{
		InitFunc: func(_ *curre.FunctionalComponent, _ context.Context) error {
			app = gin.Default()
			app.UseH2C = true

			healthRoute := app.Group("/health")
			healthRoute.GET("/ready", func(c *gin.Context) {
				c.JSON(http.StatusOK, gin.H{
					"message": "healthy",
				})
			})

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
			if server != nil {
				server.Shutdown(ctx)
			}
			return nil
		},
	})
}
