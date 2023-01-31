#!/bin/bash
docker-compose exec db bash -c "psql -U postgres -d proflex < /sql_scripts/drop_all.sql"
