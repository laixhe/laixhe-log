package biz

import "golang_log/library/api-demo/app/models"

type Biz struct {
	data *models.Data
}

func NewBiz() *Biz {
	return &Biz{
		data: models.NewData(),
	}
}
