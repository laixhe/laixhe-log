package main

import (
	"fmt"
	"os"

	"github.com/meilisearch/meilisearch-go"
)

func main() {
	client := meilisearch.New("http://localhost:7700", meilisearch.WithAPIKey("foobar"))

	// 索引
	index := client.Index("movies")

	// 添加文档
	documents := []map[string]interface{}{
		{"id": 1, "title": "Carol", "genres": []string{"Romance", "Drama"}},
		{"id": 2, "title": "Wonder Woman", "genres": []string{"Action", "Adventure"}},
		{"id": 3, "title": "Life of Pi", "genres": []string{"Adventure", "Drama"}},
		{"id": 4, "title": "Mad Max: Fury Road", "genres": []string{"Adventure", "Science Fiction"}},
		{"id": 5, "title": "Moana", "genres": []string{"Fantasy", "Action"}},
		{"id": 6, "title": "Philadelphia", "genres": []string{"Drama"}},
	}
	task, err := index.AddDocuments(documents)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	fmt.Println(task.TaskUID)

	// 基本搜索
	searchRes, err := index.Search("philoudelphia", &meilisearch.SearchRequest{Limit: 10})
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	fmt.Println(searchRes.Hits)
}
