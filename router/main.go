package router

import (
	"github.com/MephistoSolsist/win-panel/util"
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

func SetupRouter() *gin.Engine {
	gin.SetMode(gin.ReleaseMode)
	r := gin.Default()
	r.Use(cors.Default())

	r.GET("/ping", func(c *gin.Context) {
		c.String(200, "pong")
	})
	r.GET("/disk", func(c *gin.Context) {
		DiskMessageList := util.GetDiskUsage()
		if DiskMessageList == nil {
			c.JSON(500, gin.H{"error": "Could not get disk usage"})
		}
		c.JSON(200, DiskMessageList)
	})

	return r
}
