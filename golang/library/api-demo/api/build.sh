rm *.go

protoc -I . -I ../third-party --go_out=. *.proto
