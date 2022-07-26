#!/usr/bin/env bash
set -x
set -eo pipefail

# Launch the Redis database using Docker.
docker run \
  -p "6379:6379" \
  -d \
  redis:6

>&2 echo "The Redis database has been created, ready to go!"
