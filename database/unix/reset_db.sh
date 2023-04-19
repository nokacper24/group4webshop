#!/bin/bash
docker compose down
docker compose up -d
sleep 5
docker compose exec db bash -c "psql -U postgres -d proflex < /sql_scripts/init.sql"