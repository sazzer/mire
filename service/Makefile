.DEFAULT_GOAL := run

clean:
	rm -rf output

prepare:
	go get ./...
	
run: prepare check
	go run cmd/mire/*

test: prepare check
	go test ./...

check: prepare
	golangci-lint run
	
build: prepare test
	go build -o output/mire cmd/mire/*
