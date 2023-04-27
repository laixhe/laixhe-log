package utils

import (
	"errors"
	"math"
	"net"
)

// IPStringToInt 把 ip 字符串转为数值
func IPStringToInt(ipaddr string) (uint32, error) {
	parseIP := net.ParseIP(ipaddr)
	if parseIP == nil {
		return 0, errors.New("invalid ip format")
	}
	ip4 := parseIP.To4()
	if ip4 == nil {
		return 0, errors.New("invalid ipv4 format")
	}
	return uint32(ip4[3]) | uint32(ip4[2])<<8 | uint32(ip4[1])<<16 | uint32(ip4[0])<<24, nil
}

// IPIntToString 把数值转为 ip 字符串
func IPIntToString(i uint32) (string, error) {
	if i > math.MaxUint32 {
		return "", errors.New("beyond the scope of ipv4")
	}
	ip := make(net.IP, net.IPv4len)
	ip[0] = byte(i >> 24)
	ip[1] = byte(i >> 16)
	ip[2] = byte(i >> 8)
	ip[3] = byte(i)
	return ip.String(), nil
}

// IPToInt 把 net.IP 转为数值
func IPToInt(ip net.IP) (uint32, error) {
	b := ip.To4()
	if b == nil {
		return 0, errors.New("invalid ipv4 format")
	}
	return uint32(b[3]) | uint32(b[2])<<8 | uint32(b[1])<<16 | uint32(b[0])<<24, nil
}

// IntToIP 把数值转为 net.IP
func IntToIP(i uint32) (net.IP, error) {
	if i > math.MaxUint32 {
		return nil, errors.New("beyond the scope of ipv4")
	}
	ip := make(net.IP, net.IPv4len)
	ip[0] = byte(i >> 24)
	ip[1] = byte(i >> 16)
	ip[2] = byte(i >> 8)
	ip[3] = byte(i)
	return ip, nil
}
