#!/usr/bin/env bash
set -x
set -eo pipefail

# Check that dependencies are present before continuing
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed"
  exit 1
fi
if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed"
  exit 1
fi

# If a user is already set, use it. Otherwise, use "postgres"
DB_USER=${POSTGRES_USER:=postgres}
# If a password is already set, use it. Otherwise, use "password"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# If a default database is already set, use it. Otherwise, use "newsletter"
DB_NAME="${POSTGRES_DB:=newsletter}"
# If a port is already set, use it. Otherwise, use 5432
DB_PORT="${POSTGRES_PORT:=5432}"

# Launch postgres using Docker
if [[ -z "${SKIP_DOCKER}" ]]; then
  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
  #             ^^^^ Maximum number of connections increased for testing purposes
fi

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"