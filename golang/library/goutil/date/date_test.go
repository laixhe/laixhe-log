package date

import (
	"fmt"
	"testing"
)

func TestNewTime(t *testing.T) {
	te := NewTime()
	fmt.Println(te.Unix())
	fmt.Println(te.StartUnix())
	fmt.Println(te.EndUnix())
	fmt.Println(te.DateFormat())
}

func TestNewTime_YesterdayTime(t *testing.T) {
	te := NewTime().YesterdayTime()
	fmt.Println(te.Unix())
	fmt.Println(te.StartUnix())
	fmt.Println(te.EndUnix())
	fmt.Println(te.DateFormat())
}

func TestNewTime_TomorrowTime(t *testing.T) {
	te := NewTime().TomorrowTime()
	fmt.Println(te.Unix())
	fmt.Println(te.StartUnix())
	fmt.Println(te.EndUnix())
	fmt.Println(te.DateFormat())
}

func TestNewTime_WeekStartTime(t *testing.T) {
	te := NewTime().WeekStartTime()
	fmt.Println(te.Unix())
	fmt.Println(te.StartUnix())
	fmt.Println(te.EndUnix())
	fmt.Println(te.DateFormat())
}

func TestNewTime_WeekEndTime(t *testing.T) {
	te := NewTime().WeekEndTime()
	fmt.Println(te.Unix())
	fmt.Println(te.StartUnix())
	fmt.Println(te.EndUnix())
	fmt.Println(te.DateFormat())
}

func TestNewTime_MonthStartTime(t *testing.T) {
	te := NewTime().MonthStartTime()
	fmt.Println(te.Unix())
	fmt.Println(te.StartUnix())
	fmt.Println(te.EndUnix())
	fmt.Println(te.DateFormat())
}

func TestNewTime_MonthEndTime(t *testing.T) {
	te := NewTime().MonthEndTime()
	fmt.Println(te.Unix())
	fmt.Println(te.StartUnix())
	fmt.Println(te.EndUnix())
	fmt.Println(te.DateFormat())
}

func TestNewTime_YearStartTime(t *testing.T) {
	te := NewTime().YearStartTime()
	fmt.Println(te.Unix())
	fmt.Println(te.StartUnix())
	fmt.Println(te.EndUnix())
	fmt.Println(te.DateFormat())
}

func TestNewTime_YearEndTime(t *testing.T) {
	te := NewTime().YearEndTime()
	fmt.Println(te.Unix())
	fmt.Println(te.StartUnix())
	fmt.Println(te.EndUnix())
	fmt.Println(te.DateFormat())
}

func TestNewTimeUnix(t *testing.T) {
	te := NewTimeUnix(-10611244123)
	fmt.Println(te.Unix())
	fmt.Println(te.StartUnix())
	fmt.Println(te.EndUnix())
	fmt.Println(te.DateFormat())
}

func TestNewTimeParse(t *testing.T) {
	te,err := NewTimeParse("2006-2-29")
	if err != nil {
		fmt.Println("err:", err)
		return
	}
	fmt.Println(te.Unix())
	fmt.Println(te.StartUnix())
	fmt.Println(te.EndUnix())
	fmt.Println(te.DateFormat())
}

func TestNewTime_Add(t *testing.T) {
	te := NewTime().Add(120).DateFormat()
	fmt.Println(te)
}

func TestNewTime_TimeFormat(t *testing.T) {
	te := NewTime().Add(180).TimeFormat()
	fmt.Println(te)
}