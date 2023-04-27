package service

import "golang_log/library/api-demo/app/biz"

type Service struct {
	biz *biz.Biz
}

func NewService() *Service {
	return &Service{
		biz: biz.NewBiz(),
	}
}
