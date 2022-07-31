package main

import (
	"fmt"
	"net/http"

	"github.com/labstack/echo/v4"
)

func main() {
	server := echo.New()

	server.GET("/", Greet)

	server.Start(":8080")
}

func Greet(ctx echo.Context) error {
	name := ctx.QueryParam("name")
	if name == "" {
		name = "World"
	}
	greeting := fmt.Sprintf("Hello, %s!\n", name)

	return ctx.String(http.StatusOK, greeting)
}
