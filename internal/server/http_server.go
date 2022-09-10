package server

import (
	"context"
	"net/http"

	"git.front.kjuulh.io/kjuulh/curre"
	"git.front.kjuulh.io/kjuulh/kraken/internal/serverdeps"
	"github.com/aerogo/aero"
)

func NewHttpServer(deps *serverdeps.ServerDeps) curre.Component {
	return curre.NewFunctionalComponent(&curre.FunctionalComponent{
		StartFunc: func(fc *curre.FunctionalComponent, ctx context.Context) error {
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

func NewAeroHttpServer(deps *serverdeps.ServerDeps) curre.Component {
	var app *aero.Application

	return curre.NewFunctionalComponent(&curre.FunctionalComponent{
		InitFunc: func(_ *curre.FunctionalComponent, _ context.Context) error {
			app = aero.New()
			app.Config.Ports = aero.PortConfiguration{
				HTTP:  3000,
				HTTPS: 3443,
			}

			app.Router().Add(http.MethodGet, "/health/ready", func(ctx aero.Context) error {
				ctx.JSON(struct {
					Message string `json:"message"`
				}{Message: "healthy"})
				return nil
			})

			return nil
		},
		StartFunc: func(_ *curre.FunctionalComponent, _ context.Context) error {
			if app != nil {
				app.Run()
			}
			return nil
		},
		StopFunc: func(_ *curre.FunctionalComponent, _ context.Context) error {
			if app != nil {
				app.Shutdown()
			}
			return nil
		},
	})
}
