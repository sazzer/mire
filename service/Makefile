.DEFAULT_GOAL := run

clean:
	rm -rf output

prepare:
	go get ./...
	
run: prepare
	go run cmd/mire/*

test: prepare
	go test ./...

build: prepare
	go build -o output/mire cmd/mire/*
