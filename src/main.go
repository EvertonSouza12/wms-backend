package main

import (
	"log"
	
    "wms-backend/config"
	"wms-backend/routes"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/logger"
)

func main() {
	config.ConnectDB()
	routes.SetupRoutes(app)

	log.Fatal(app.Listen(":3000"))
}
