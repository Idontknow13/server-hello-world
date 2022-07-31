package main

import (
	"fmt"
	"log"

	"github.com/gofiber/fiber/v2"
)

func main() {
	server := fiber.New()

	server.Get("/", Greet)

	log.Fatal(server.Listen(":8080"))
}

func Greet(ctx *fiber.Ctx) error {
	name := ctx.Query("name", "World") // /?name={name} -> Hello, {name}!
	greeting := fmt.Sprintf("Hello, %s!\n", name)

	return ctx.Status(200).SendString(greeting)
}
