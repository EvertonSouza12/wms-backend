package handlers

import (
	"wms-backend/config"
	"wms-backend/models"

	"github.com/gofiber/fiber/v2"
	"github.com/google/uuid"
)


func GetAllProducts(c *fiber.Ctx) error {
	var products []models.Product
	if result := config.DB.Find(&products); result.Error != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Failed to retrieve products",
		})
	}
	return c.Status(fiber.StatusOK).JSON(products)
}