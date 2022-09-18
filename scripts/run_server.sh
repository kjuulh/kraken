#!/bin/bash

set -e

export $(cat .env | xargs)

go run cmd/server/server.go start
