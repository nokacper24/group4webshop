#!/bin/bash
docker compose exec db-prod bash -c "psql -U postgres -d proflex < /sql_scripts/init.sql"
