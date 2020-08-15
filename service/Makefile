.DEFAULT_GOAL := allTests

clean:
	rm -rf output

prepare:
	go get ./...
	
run: prepare check
	go run cmd/mire/*

test: prepare check
	go test ./internal/...

testFast: prepare check
	go test ./internal/... -short

allTests: prepare check test

check: prepare
	golangci-lint run
	
build: prepare test clean
	go build -o output/mire cmd/mire/*
