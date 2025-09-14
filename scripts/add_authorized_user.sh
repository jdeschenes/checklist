#!/usr/bin/env bash

set -eo pipefail

# Database connection parameters
DB_PORT="${DB_PORT:=5432}"
APP_USER="${APP_USER:=app}"
APP_USER_PWD="${APP_USER_PWD:=secret}"
APP_DB_NAME="${APP_DB_NAME:=newdbname}"

# User to add
USER_EMAIL="${USER_EMAIL:=deschenes.j.m@gmail.com}"

DATABASE_URL="postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME}?sslmode=disable"

echo "Adding authorized user ${USER_EMAIL} to the database..."

# Check if running in Docker or local environment
if [[ -z "${SKIP_DOCKER}" ]]
then
    CONTAINER_NAME="postgres"

    # Insert user if not exists (user_id will be auto-generated)
    INSERT_USER_QUERY="INSERT INTO users (email) VALUES ('${USER_EMAIL}') ON CONFLICT (email) DO NOTHING;"
    docker exec -it "${CONTAINER_NAME}" psql -d ${APP_DB_NAME} -U "${APP_USER}" -c "${INSERT_USER_QUERY}"

    echo "User ${USER_EMAIL} has been added to the database (or already exists)"
else
    # For local PostgreSQL installations
    psql "${DATABASE_URL}" -c "INSERT INTO users (email) VALUES ('${USER_EMAIL}') ON CONFLICT (email) DO NOTHING;"
    echo "User ${USER_EMAIL} has been added to the database (or already exists)"
fi