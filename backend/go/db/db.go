package db

import (
	"log"

	"gorm.io/driver/mysql"
	"gorm.io/gorm"
)

var DB *gorm.DB

// データベース接続を初期化する
func InitDB() {
	dsn := "todo_user:password@tcp(localhost:3306)/todo_db?charset=utf8mb4&parseTime=True&loc=Local"

	db, err := gorm.Open(mysql.Open(dsn), &gorm.Config{})
	// 接続エラーなら落とす
	if err != nil {
		log.Fatal("failed to connect database:", err)
		// panic("failed to connect database")  ←panic関数でも強制終了させられるらしい
	}

	// グローバル変数にDB接続情報を保存
	DB = db
}
