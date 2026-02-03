package handlers

import (
	"net/http"

	"github.com/gin-gonic/gin"

	"todo-go/db"
	"todo-go/models"
	"todo-go/requests"
)

func GetTodos(c *gin.Context) {
	var todos []models.Todo

	if err := db.DB.Find(&todos).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"detail": "failed to get todos"})
		return
	}

	c.JSON(http.StatusOK, todos)
}

func CreateTodo(c *gin.Context) {
	var req requests.CreateTodoRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"detail": "invalid request"})
		return
	}

	todo := models.Todo{
		Title: req.Title,
	}

	if err := db.DB.Create(&todo).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"detail": "failed to create todo"})
		return
	}

	c.JSON(http.StatusOK, todo)
}

func UpdateTodo(c *gin.Context) {
	var todo models.Todo

	if err := db.DB.First(&todo, c.Param("id")).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"detail": "Todo not found"})
		return
	}

	var req requests.UpdateTodoRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"detail": "invalid request"})
		return
	}

	todo.Completed = req.Completed

	if err := db.DB.Save(&todo).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"detail": "failed to update todo"})
		return
	}

	c.JSON(http.StatusOK, todo)
}

func DeleteTodo(c *gin.Context) {
	if err := db.DB.Delete(&models.Todo{}, c.Param("id")).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"detail": "Todo not found"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "deleted"})
}
