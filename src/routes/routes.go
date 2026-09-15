package routes

import (
	"wms-backend/handlers"
	"github.com/gofiber/fiber/v2"
)

func SetupRoutes(app *fiber.App) {
	api := app.Group("/api")
	api.Get("/products", handlers.GetAllProducts)
}