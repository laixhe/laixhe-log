package main

import (
	"net/http"
	"testing"

	"github.com/quic-go/quic-go/http3"
)

// HTTP/3 客户端
func TestHttp3Client(t *testing.T) {
	client := &http.Client{
		Transport: &http3.Transport{},
	}

	resp, err := client.Get("https://example.com")
	if err != nil {
		t.Fatal(err)
		return
	}
	defer resp.Body.Close()
}
