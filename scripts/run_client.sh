#!/bin/bash

set -e

current_branch=$(git branch --show-current)

go run cmd/kraken/kraken.go process --actions-repo "git@git.front.kjuulh.io:kjuulh/kraken.git" --branch "$current_branch" --path "_examples/actions/write_a_readme"
go run cmd/kraken/kraken.go process --actions-repo "git@git.front.kjuulh.io:kjuulh/kraken.git" --branch "$current_branch" --path "_examples/queries/scrape_readme"
