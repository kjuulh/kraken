#!/bin/bash

set -e

current_branch=$(git branch --show-current)

export $(cat .env | xargs)

#go run cmd/octopush/octopush.go process --actions-repo "git@git.front.kjuulh.io:kjuulh/octopush.git" --branch "$current_branch" --path "_examples/actions/write_a_readme"
go run cmd/octopush/octopush.go process \
  --actions-repo "git@git.front.kjuulh.io:kjuulh/octopush.git"\
  --branch "$current_branch" \
  --path "_examples/actions/add_releaserc"
