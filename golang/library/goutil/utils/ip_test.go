package utils

import (
	"fmt"
	"net"
	"testing"
)

func TestIPStringToInt(t *testing.T) {

	//{"127.0.0.1", 2130706433}
	//{"0.0.0.0", 0}
	//{"255.255.255.255", 4294967295}
	//{"192.168.1.1", 3232235777}
	//{"113.81.196.213", 1901184213}

	data := []string{"127.0.0.1", "0.0.0.0", "255.255.255.255", "192.168.1.1", "113.81.196.213"}

	for _, v := range data {

		ipInt, err := IPStringToInt(v)
		if err != nil {
			t.Error(err)
			continue
		}

		fmt.Println(ipInt)

	}

}

func TestIPIntToString(t *testing.T) {

	//{"127.0.0.1", 2130706433}
	//{"0.0.0.0", 0}
	//{"255.255.255.255", 4294967295}
	//{"192.168.1.1", 3232235777}
	//{"113.81.196.213", 1901184213}

	data := []uint32{2130706433, 0, 4294967295, 3232235777, 1901184213}

	for _, v := range data {

		ipString, err := IPIntToString(v)
		if err != nil {
			t.Error(err)
			continue
		}

		fmt.Println(ipString)

	}

}

func TestIPToInt(t *testing.T) {

	//{"127.0.0.1", 2130706433}
	//{"0.0.0.0", 0}
	//{"255.255.255.255", 4294967295}
	//{"192.168.1.1", 3232235777}
	//{"113.81.196.213", 1901184213}

	data := []string{"127.0.0.1", "0.0.0.0", "255.255.255.255", "192.168.1.1", "113.81.196.213"}

	for _, v := range data {

		ipInt, err := IPToInt(net.ParseIP(v))
		if err != nil {
			t.Error(err)
			continue
		}

		fmt.Println(ipInt)

	}

}

func TestIntToIP(t *testing.T) {

	//{"127.0.0.1", 2130706433}
	//{"0.0.0.0", 0}
	//{"255.255.255.255", 4294967295}
	//{"192.168.1.1", 3232235777}
	//{"113.81.196.213", 1901184213}

	data := []uint32{2130706433, 0, 4294967295, 3232235777, 1901184213}

	for _, v := range data {

		ipString, err := IntToIP(v)
		if err != nil {
			t.Error(err)
			continue
		}

		fmt.Println(ipString)

	}

}
