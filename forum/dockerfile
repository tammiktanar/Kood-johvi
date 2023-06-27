# syntax=docker/dockerfile:1
##
## Build
##
FROM golang:1.17 AS build
# Dependancies
WORKDIR /go/src/forum
COPY go.mod ./
COPY go.sum ./
COPY cmd/forum/main.go ./
RUN go mod download
# Copy source files
COPY . ./
# Build
RUN go build -o ./
##
## Deploy
##

EXPOSE 8000
ENTRYPOINT ["./forum"]