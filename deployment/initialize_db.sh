#!/bin/bash

source .env

if [ -z "$BACKEND_USR_PASS" ]; then
  echo "Error: BACKEND_USR_PASS environmental variable is not set.
  You can set it in the .env file"
  exit 1
fi

docker compose exec db bash -c "psql -U postgres -d proflex < /sql_scripts/initialize.sql"
docker compose exec db bash -c "psql -U postgres -d proflex < /sql_scripts/populate_prod.sql"
echo "CREATE USER backend_user WITH PASSWORD '$BACKEND_USR_PASS';" | docker compose exec -T db psql -U postgres -d proflex
echo "GRANT SELECT, UPDATE, INSERT, DELETE ON ALL TABLES IN SCHEMA public TO backend_user;" | docker compose exec -T db psql -U postgres -d proflex
echo "GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO backend_user;" | docker compose exec -T db psql -U postgres -d proflex
