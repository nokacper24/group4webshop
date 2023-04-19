@echo off
docker-compose exec db bash -c "psql -U postgres -d proflex < /sql_scripts/populate_dev.sql"