package array

// FilterRepeatString 过滤重复数据 string (一维数组)
func FilterRepeatString(s []string) []string {
	if len(s) == 0 {
		return []string{}
	}

	dataMap := make(map[string]bool)
	for _, v := range s {
		dataMap[v] = true
	}

	data := make([]string, 0, len(dataMap))
	for k := range dataMap {
		data = append(data, k)
	}

	return data
}

// FilterRepeatInt8 过滤重复数据 int8 (一维数组)
func FilterRepeatInt8(i []int8) []int8 {
	if len(i) == 0 {
		return []int8{}
	}

	dataMap := make(map[int8]bool)
	for _, v := range i {
		dataMap[v] = true
	}

	data := make([]int8, 0, len(dataMap))
	for k := range dataMap {
		data = append(data, k)
	}

	return data
}

// FilterRepeatUint8 过滤重复数据 uint8 (一维数组)
func FilterRepeatUint8(i []uint8) []uint8 {
	if len(i) == 0 {
		return []uint8{}
	}

	dataMap := make(map[uint8]bool)
	for _, v := range i {
		dataMap[v] = true
	}

	data := make([]uint8, 0, len(dataMap))
	for k := range dataMap {
		data = append(data, k)
	}

	return data
}

// FilterRepeatInt 过滤重复数据 int (一维数组)
func FilterRepeatInt(i []int) []int {
	if len(i) == 0 {
		return []int{}
	}

	dataMap := make(map[int]bool)
	for _, v := range i {
		dataMap[v] = true
	}

	data := make([]int, 0, len(dataMap))
	for k := range dataMap {
		data = append(data, k)
	}

	return data
}

// FilterRepeatUint 过滤重复数据 uint (一维数组)
func FilterRepeatUint(i []uint) []uint {
	if len(i) == 0 {
		return []uint{}
	}

	dataMap := make(map[uint]bool)
	for _, v := range i {
		dataMap[v] = true
	}

	data := make([]uint, 0, len(dataMap))
	for k := range dataMap {
		data = append(data, k)
	}

	return data
}

// FilterRepeatInt32 过滤重复数据 int32 (一维数组)
func FilterRepeatInt32(i []int32) []int32 {
	if len(i) == 0 {
		return []int32{}
	}

	dataMap := make(map[int32]bool)
	for _, v := range i {
		dataMap[v] = true
	}

	data := make([]int32, 0, len(dataMap))
	for k := range dataMap {
		data = append(data, k)
	}

	return data
}

// FilterRepeatUint32 过滤重复数据 uint32 (一维数组)
func FilterRepeatUint32(i []uint32) []uint32 {
	if len(i) == 0 {
		return []uint32{}
	}

	dataMap := make(map[uint32]bool)
	for _, v := range i {
		dataMap[v] = true
	}

	data := make([]uint32, 0, len(dataMap))
	for k := range dataMap {
		data = append(data, k)
	}

	return data
}

// FilterRepeatInt64 过滤重复数据 int64 (一维数组)
func FilterRepeatInt64(i []int64) []int64 {
	if len(i) == 0 {
		return []int64{}
	}

	dataMap := make(map[int64]bool)
	for _, v := range i {
		dataMap[v] = true
	}

	data := make([]int64, 0, len(dataMap))
	for k := range dataMap {
		data = append(data, k)
	}

	return data
}

// FilterRepeatUint64 过滤重复数据 uint64 (一维数组)
func FilterRepeatUint64(i []uint64) []uint64 {
	if len(i) == 0 {
		return []uint64{}
	}

	dataMap := make(map[uint64]bool)
	for _, v := range i {
		dataMap[v] = true
	}

	data := make([]uint64, 0, len(dataMap))
	for k := range dataMap {
		data = append(data, k)
	}

	return data
}

// InArrayString 判断元素是否包含(字符串)(一维数组)
func InArrayString(s string, arr []string) bool {

	for _, val := range arr {
		if s == val {
			return true
		}
	}

	return false
}

// InArrayInt8 判断元素是否包含(整数)(一维数组)
func InArrayInt8(i int8, arr []int8) bool {

	for _, val := range arr {
		if i == val {
			return true
		}
	}

	return false
}

// InArrayUint8 判断元素是否包含(整数)(一维数组)
func InArrayUint8(i uint8, arr []uint8) bool {

	for _, val := range arr {
		if i == val {
			return true
		}
	}

	return false
}

// InArrayInt 判断元素是否包含(整数)(一维数组)
func InArrayInt(i int, arr []int) bool {

	for _, val := range arr {
		if i == val {
			return true
		}
	}

	return false
}

// InArrayUint 判断元素是否包含(整数)(一维数组)
func InArrayUint(i uint, arr []uint) bool {

	for _, val := range arr {
		if i == val {
			return true
		}
	}

	return false
}

// InArrayInt32 判断元素是否包含(整数32)(一维数组)
func InArrayInt32(i int32, arr []int32) bool {

	for _, val := range arr {
		if i == val {
			return true
		}
	}

	return false
}

// InArrayUint32 判断元素是否包含(整数32)(一维数组)
func InArrayUint32(i uint32, arr []uint32) bool {

	for _, val := range arr {
		if i == val {
			return true
		}
	}

	return false
}

// InArrayInt64 判断元素是否包含(整数64)(一维数组)
func InArrayInt64(i int64, arr []int64) bool {

	for _, val := range arr {
		if i == val {
			return true
		}
	}

	return false
}

// InArrayUint64 判断元素是否包含(整数64)(一维数组)
func InArrayUint64(i uint64, arr []uint64) bool {

	for _, val := range arr {
		if i == val {
			return true
		}
	}

	return false
}
