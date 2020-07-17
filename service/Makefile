.DEFAULT_GOAL := run

clean:
	rm -rf output

prepare:
	go get ./...
	
run: prepare
	go run cmd/mire/*

test: prepare
	go test ./...

check: prepare
	golangci-lint run
	
build: prepare
	go build -o output/mire cmd/mire/*
