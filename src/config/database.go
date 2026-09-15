package config

import (
	"log"
	"os"

	"wms-backend/models"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

var DB *gorm.DB

func ConnectDB() {
	dbURL := os.Getenv("DATABASE_URL")
	if dbURL == "" {
		dbURL = "host=localhost user=postgres password=postgres dbname=wms port=5432 sslmode=disable"
	}

var err error
	DB, err = gorm.Open(postgres.Open(dbURL), &gorm.Config{})
	if err != nil {
		log.Fatal("Failed to connect to database:", err)
	}

err = DB.AutoMigrate(&models.Product{})
	if err != nil {
		log.Fatal("Failed to migrate database:", err)
	}
	log.Println("Database connection established and migrated successfully.")
}