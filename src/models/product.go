package models

import (
	"time"
)


type Product struct {
	ID          uint      `gorm:"primaryKey" json:"id"`
	SKU         string    `gorm:"type:varchar(100);Unique;not null" json:"sku"`
	Description string    `gorm:"type:text" json:"description"`
	Barcode     float64   `gorm:"type:varchar(100);not null" json:"barcode"`
	MinStock    int       `gorm:"type:int;not null" json:"min_stock"`
	CreatedAt   time.Time `gorm:"autoCreateTime" json:"created_at"`
	UpdatedAt   time.Time `gorm:"autoUpdateTime" json:"updated_at"`
}