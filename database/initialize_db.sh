#!/bin/bash
docker compose down
docker compose up -d
echo Docker compose done, waiting for 5 seconds...
echo If it wails, increase the sleep time.
sleep 5
docker compose exec db bash -c "psql -U postgres -d proflex < /sql_scripts/initialize.sql"
echo "CREATE USER backend_user WITH PASSWORD 'password';" | docker compose exec -T db psql -U postgres -d proflex
echo "GRANT SELECT, UPDATE, INSERT, DELETE ON ALL TABLES IN SCHEMA public TO backend_user;" | docker compose exec -T db psql -U postgres -d proflex
echo "GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO backend_user;" | docker compose exec -T db psql -U postgres -d proflex
