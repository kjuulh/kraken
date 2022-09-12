package api

import (
	"git.front.kjuulh.io/kjuulh/kraken/internal/serverdeps"
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
)

func BuildApi(logger *zap.Logger, app *gin.Engine, deps *serverdeps.ServerDeps) {
	HealthRoute(app)
	CommandRoute(logger, app, deps)
}
