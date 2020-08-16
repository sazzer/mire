.DEFAULT_GOAL := allTests

clean:
	rm -rf output

tidy:
	go get -u ./...
	go mod tidy
	
prepare:
	go get ./...
	
run: prepare check
	go run cmd/mire/*

test: prepare check
	go test ./internal/...

testIntegration: prepare check
	go test ./integration/...

testFast: prepare check
	go test ./internal/... -short

allTests: prepare check test testIntegration

check: prepare
	golangci-lint run --allow-parallel-runners
	
build: prepare test clean
	go build -o output/mire cmd/mire/*
