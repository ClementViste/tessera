#!/usr/bin/env bash
set -x
set -eo pipefail

# Launch the Redis instance using Docker.
docker run \
  -p "6379:6379" \
  -d \
  redis:6

>&2 echo "The Redis instance has been created, ready to go!"