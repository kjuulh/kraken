package api

import (
	"context"
	"net/http"

	"git.front.kjuulh.io/kjuulh/kraken/internal/commands"
	"git.front.kjuulh.io/kjuulh/kraken/internal/serverdeps"
	"git.front.kjuulh.io/kjuulh/kraken/internal/services/jobs"
	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
	"go.uber.org/zap"
)

func CommandRoute(logger *zap.Logger, app *gin.Engine, deps *serverdeps.ServerDeps) {
	commandRoute := app.Group("commands")
	commandRoute.POST("processRepos", func(c *gin.Context) {
		type processReposRequest struct {
			RepositoryUrls []string `json:"repositoryUrls"`
		}
		var request processReposRequest
		err := c.BindJSON(&request)
		if err != nil {
			logger.Info("could not bind request", zap.String("request", "processRepo"), zap.Error(err))
			c.AbortWithStatus(http.StatusBadRequest)
			return
		}

		jobId := uuid.New().String()

		go func(repositoryUrls []string, jobId string) {
			ctx := context.WithValue(context.Background(), jobs.JobId{}, jobId)
			processRepos := commands.NewProcessRepos(logger, deps)
			err = processRepos.Process(ctx, repositoryUrls)
		}(request.RepositoryUrls, jobId)

		c.Status(http.StatusAccepted)
	})
}
