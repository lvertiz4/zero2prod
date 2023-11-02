#!/usr/bin/env bash
set -x
set -eo pipefail

# TODO: Figure out where these are listed on MacOS, usually at /usr/local/Cellar/
# but the terminal response was "zsh: ./scripts/init_db.sh: bad interpreter: /usr/local/Cellar/: no such file or directory"
# if ! [ -x "$(command -v psql)"]; then
#     echo >&2 "Error: psql is not installed."
#     exit 1
# fi

# if ! [ -x "$(command -v sqlx)"]; then
#     echo >&2 "Error: sqlx is not installed."
#     echo >&2 "Use:"
#     echo >&2 "      cargo install --version='~0.7' sqlx-cli \
#     --no-default-features --features rustls,postgres"
#     echo >&2 "to install it."
#     exit 1
# fi

#Check if a custom user has been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"
#Check if a custom password has been set, otherwise, default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
#Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
#Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

#Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
#Launch postgres using Docker
docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_DB} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
    #Increases maximum number of connections for testing purposes
fi

#Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do 
    >&2 echo "Postgres is still unavailable. It's sleeping."
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"