@echo off
docker-compose up -d
timeout /t 5
docker-compose exec db bash -c "psql -U postgres -c 'CREATE DATABASE proflex;'"