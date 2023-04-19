#!/bin/bash
docker compose down
docker compose up -d
sleep 5
docker compose exec db bash -c "psql -U postgres -d proflex < /sql_scripts/initialize.sql"
docker compose exec db bash -c "psql -U postgres -d proflex < /sql_scripts/populate_dev.sql"

docker compose exec db bash -c "psql -U postgres -d proflex -c 'CREATE USER backend_user WITH PASSWORD \'password\';'" # escape the single quotes, fix this TODO
docker compose exec db bash -c "psql -U postgres -d proflex -c 'GRANT SELECT, UPDATE, INSERT, DELETE ON ALL TABLES IN SCHEMA public TO backend_user;'"
docker compose exec db bash -c "psql -U postgres -d proflex -c 'GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO backend_user;'"

# CREATE USER backend_user WITH PASSWORD 'password'; 
# GRANT SELECT, UPDATE, INSERT, DELETE ON ALL TABLES IN SCHEMA public TO backend_user; 
# GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO backend_user;"