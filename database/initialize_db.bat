@echo off
docker-compose down
docker-compose up -d
timeout /t 5
docker-compose exec db bash -c "psql -U postgres -d proflex < /sql_scripts/init.sql"

docker compose exec db bash -c "psql -U postgres -d proflex -c 'CREATE USER backend_user WITH PASSWORD 'password'; GRANT SELECT, UPDATE, INSERT, DELETE ON ALL TABLES IN SCHEMA public TO backend_user; GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO backend_user;'"