#!/bin/bash

set -e

git remote add github git@github.com:kjuulh/kraken.git || true

git push -f github main
