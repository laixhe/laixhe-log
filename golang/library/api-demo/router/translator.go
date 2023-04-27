package router

import (
	"fmt"

	"github.com/gin-gonic/gin/binding"
	"github.com/go-playground/locales/en"
	"github.com/go-playground/locales/zh"
	translator "github.com/go-playground/universal-translator"
	"github.com/go-playground/validator/v10"
	enTranslations "github.com/go-playground/validator/v10/translations/en"
	zhTranslations "github.com/go-playground/validator/v10/translations/zh"
)

var trans translator.Translator

// ValidatorTranslator Validator(表单验证)多语言提示文本
func ValidatorTranslator(language string) (err error) {
	// 修改 gin 框架中 Validator 引擎属性，实现自定制
	if v, ok := binding.Validator.Engine().(*validator.Validate); ok {
		zhT := zh.New() // 中文
		enT := en.New() // 英文
		// 第一个参数是备用语言，后面参数是应该支持多个语言
		universalTranslator := translator.New(enT, zhT, enT)
		// language 通常取决于 http 请求 Accept-language
		var is bool
		trans, is = universalTranslator.GetTranslator(language)
		if !is {
			return fmt.Errorf("uni.GetTranslator(%s) failed", language)
		}
		// 注册翻译器
		switch language {
		case "en":
			err = enTranslations.RegisterDefaultTranslations(v, trans)
		case "zh":
			err = zhTranslations.RegisterDefaultTranslations(v, trans)
		default:
			err = enTranslations.RegisterDefaultTranslations(v, trans)
		}
	}
	return
}
