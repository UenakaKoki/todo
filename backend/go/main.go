package main

import (
	"todo-go/db"
	"todo-go/models"
	"todo-go/router"
)

func main() {
	db.InitDB()

	db.DB.AutoMigrate(&models.Todo{})

	r := router.SetupRouter()
	r.Run(":8000")
}
