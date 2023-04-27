package keyword

import (
	"fmt"
	"testing"
)

func TestNewSensitiveWord(t *testing.T) {
	s := NewSensitiveWord()
	s.AddSensitiveToMap([]string{
		"你好",
		"您好",
		"哈喽",
		"在吗",
	})

	fmt.Printf("%#v\n", s.ReplaceSensitiveWords("你	好123哈^&*喽"))
	fmt.Printf("%#v\n", s.ChangeSensitiveWords("你 好123哈^&*喽"))
}
