package keyword

// SensitiveInvalidWords 无效词汇
var SensitiveInvalidWords = []string{
	"1",
	"2",
	"3",
	"4",
	"5",
	"6",
	"7",
	"8",
	"9",
	"0",
	" ",
	"~",
	"",
	"!",
	"@",
	"#",
	"$",
	"%",
	"^",
	"&",
	"*",
	"(",
	")",
	"_",
	"-",
	"+",
	"=",
	"?",
	"<",
	">",
	".",
	"—",
	"，",
	"。",
	"/",
	",",
	"|",
	"《",
	"》",
	"？",
	";",
	":",
	"：",
	"'",
	"‘",
	"；",
	"“",
	"\"",
	"\t",
	"\n",
}

// SensitiveWord 敏感词汇过滤
type SensitiveWord struct {
	InvalidWord       map[string]interface{} // 无效词汇，不参与敏感词汇判断直接忽略
	DataSensitiveWord map[string]interface{} // 敏感词汇
}

// NewSensitiveWord 敏感词汇过滤
func NewSensitiveWord() *SensitiveWord {
	sensitiveWord := &SensitiveWord{
		InvalidWord:       make(map[string]interface{}),
		DataSensitiveWord: make(map[string]interface{}),
	}

	// 写入无效词汇
	if len(SensitiveInvalidWords) > 0 {
		for _, v := range SensitiveInvalidWords {
			sensitiveWord.InvalidWord[v] = nil
		}
	}
	return sensitiveWord
}

// AddSensitiveToMap 生成敏感词(违禁词)集合
func (s *SensitiveWord) AddSensitiveToMap(strs []string) {
	for _, v := range strs {
		strRune := []rune(v)
		nowMap := s.DataSensitiveWord
		for i := 0; i < len(strRune); i++ {
			if _, ok := nowMap[string(strRune[i])]; !ok { // 如果该key不存在，
				thisMap := make(map[string]interface{})
				thisMap["isEnd"] = false
				nowMap[string(strRune[i])] = thisMap
				nowMap = thisMap
			} else {
				nowMap = nowMap[string(strRune[i])].(map[string]interface{})
			}
			if i == len(strRune)-1 {
				nowMap["isEnd"] = true
			}
		}
	}
}

// ReplaceSensitiveWords 将敏感词替换为 *
func (s *SensitiveWord) ReplaceSensitiveWords(txt string) string {
	str := []rune(txt)
	nowMap := s.DataSensitiveWord
	start := -1
	tag := -1
	for i := 0; i < len(str); i++ {
		if _, ok := s.InvalidWord[(string(str[i]))]; ok {
			continue // 如果是无效词汇直接跳过
		}
		if thisMap, ok := nowMap[string(str[i])].(map[string]interface{}); ok {
			// 记录敏感词第一个文字的位置
			tag++
			if tag == 0 {
				start = i
			}

			// 判断是否为敏感词的最后一个文字
			if isEnd, _ := thisMap["isEnd"].(bool); isEnd {
				// 将敏感词的第一个文字到最后一个文字全部替换为 *
				for y := start; y < i+1; y++ {
					str[y] = 42
				}
				// 重置标志数据
				nowMap = s.DataSensitiveWord
				start = -1
				tag = -1

			} else { // 不是最后一个，则将其包含的map赋值给nowMap
				nowMap = nowMap[string(str[i])].(map[string]interface{})
			}

		} else { // 如果敏感词不是全匹配，则终止此敏感词查找。从开始位置的第二个文字继续判断
			if start != -1 {
				i = start + 1
			}
			// 重置标志参数
			nowMap = s.DataSensitiveWord
			start = -1
			tag = -1
		}
	}

	return string(str)
}

// ChangeSensitiveWords 将检测到的敏感词提取出来
func (s *SensitiveWord) ChangeSensitiveWords(txt string) []string {
	str := []rune(txt)
	nowMap := s.DataSensitiveWord
	start := -1
	tag := -1

	var results []string
	var result []rune

	for i := 0; i < len(str); i++ {
		if _, ok := s.InvalidWord[(string(str[i]))]; ok {
			continue // 如果是无效词汇直接跳过
		}
		if thisMap, ok := nowMap[string(str[i])].(map[string]interface{}); ok {
			// 记录当前文字
			result = append(result, str[i])
			// 记录敏感词第一个文字的位置
			tag++
			if tag == 0 {
				start = i
			}

			// 判断是否为敏感词的最后一个文字
			if isEnd, _ := thisMap["isEnd"].(bool); isEnd {
				results = append(results, string(result))
			} else { // 不是最后一个，则将其包含的map赋值给nowMap
				nowMap = nowMap[string(str[i])].(map[string]interface{})
			}

		} else { // 如果敏感词不是全匹配，则终止此敏感词查找。从开始位置的第二个文字继续判断
			if start != -1 {
				i = start + 1
			}
			// 重置标志参数
			nowMap = s.DataSensitiveWord
			start = -1
			tag = -1
			result = result[:0]
		}
	}

	return results
}
