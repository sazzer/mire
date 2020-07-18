.DEFAULT_GOAL := allTests

clean:
	rm -rf output

prepare:
	go get ./...
	
run: prepare check
	go run cmd/mire/*

test: prepare check
	go test ./internal/...

integration: prepare check
	go test ./tests/...

integrationUpdate: prepare check
	UPDATE_SNAPSHOTS=true go test ./tests/...

allTests: prepare check test integration

check: prepare
	golangci-lint run
	
build: prepare test clean
	go build -o output/mire cmd/mire/*
