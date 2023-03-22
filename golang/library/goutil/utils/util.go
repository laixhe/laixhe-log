package utils

import "runtime"

// GetLineInfo 获取当前的文件名、行号、方法名
// skip 调用栈层级， 0为当前所在的位置，1为调用该方法的位置，以此类推。。。
func GetLineInfo(skip int) (fileName string, funcName string, lineNo int) {

	// pc 是程序执行指令的记数器，用于记录执行指令的位置
	pc, file, line, ok := runtime.Caller(skip)
	if ok {
		fileName = file
		funcName = runtime.FuncForPC(pc).Name() // 获取方法名
		lineNo = line
	}

	return
}

// PageNumber 计算分页数
// page    第几页
// perPage 每页的记录数
func PageNumber(page, perPage int) (limit int, offset int) {
	if page <= 0 {
		page = 1
	}
	if perPage <= 0 {
		perPage = 12
	}
	offset = (page - 1) * perPage
	limit = perPage
	return
}
