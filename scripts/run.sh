#!/bin/bash

set -e

run_server="cuddle_cli x run_server"

$run_server &

sleep 1s

cuddle_cli x run_client

sleep 5s

kill %1
