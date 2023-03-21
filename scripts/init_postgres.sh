#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if a custom user has been set, otherwise default to `postgres`.
DB_USER="${POSTGRES_USER:=postgres}"
# Check if a custom password has been set, otherwise default to `password`.
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom port has been set, otherwise default to `5432`.
DB_PORT="${POSTGRES_PORT:=5432}"
# Check if a custom database name has been set, otherwise default to `tracker`.
DB_NAME="${POSTGRES_DB:=tracker}"

# Check if the `SKIP_DOCKER` environment variable is set.
#
# Skip Docker if set, otherwise launch the Postgres instance using Docker.
if [[ -z "${SKIP_DOCKER}" ]]
then
  # Launch the Postgres database using Docker.
  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -p "${DB_PORT}":5432 \
    -e POSTGRES_DB=${DB_NAME} \
    -d postgres \
    postgres -N 1000
fi

# Keep pinging the Postgres instance until it's ready to accept commands.
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "The Postgres instance is still unavailable - sleeping."
  sleep 1
done

>&2 echo "The Postgres instance is up and running on port ${DB_PORT}, running migrations now!"

# Set the `DATABASE_URL` environment variable.
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "The database has been created and migrated, ready to go!"