package main

import (
	"fmt"

	"github.com/MephistoSolsist/win-panel/router"
)

func main() {
	port := "12345"
	r := router.SetupRouter()
	r.Run("0.0.0.0:" + port)
	fmt.Printf("Starting server at %v...", port)
}
