#!/bin/bash

set -e

docker compose -f scripts/generate_client.docker-compose.yml build
docker compose -f scripts/generate_client.docker-compose.yml up generator
docker compose -f scripts/generate_client.docker-compose.yml down
