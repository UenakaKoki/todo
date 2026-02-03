package models

type Todo struct {
	ID        uint   `json:"id" gorm:"primaryKey"`
	Title     string `json:"title" gorm:"not null"`
	Completed bool   `json:"completed" gorm:"default:false"`
}

// 先頭小文字でprivate扱いらしい
// type todo struct {
// 	ID        uint   `json:"id" gorm:"primaryKey"`
// 	Title     string `json:"title" gorm:"not null"`
// 	Completed bool   `json:"completed" gorm:"default:false"`
// }
