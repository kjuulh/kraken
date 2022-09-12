package api

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func HealthRoute(app *gin.Engine) {
	healthRoute := app.Group("/health")
	healthRoute.GET("/ready", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "healthy",
		})
	})
}
