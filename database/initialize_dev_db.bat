@echo off
set SLEEP_TIME=10

docker-compose down
docker-compose up -d

echo Docker compose done.
echo Waiting %SLEEP_TIME% seconds for the database to start...
timeout /t %SLEEP_TIME%

docker-compose exec db bash -c "psql -U postgres -d proflex < /sql_scripts/initialize.sql"
docker-compose exec db bash -c "psql -U postgres -d proflex < /sql_scripts/populate_dev.sql"
echo CREATE USER backend_user WITH PASSWORD 'password'; | docker-compose exec -T db psql -U postgres -d proflex
echo GRANT SELECT, UPDATE, INSERT, DELETE ON ALL TABLES IN SCHEMA public TO backend_user; | docker-compose exec -T db psql -U postgres -d proflex
echo GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO backend_user; | docker-compose exec -T db psql -U postgres -d proflex