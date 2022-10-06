#!/bin/bash

set -e

git remote add github git@github.com:kjuulh/octopush.git || true

git push -f github --all
