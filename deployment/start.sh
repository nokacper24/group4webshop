#!/bin/bash
SLEEP_TIME=10
source .env
docker pull ghcr.io/nokacper24/group4webshop:${IMG_TAG:-main}
docker compose down
docker compose up -d

echo "Waiting for $SLEEP_TIME seconds for the database to start..."
sleep $SLEEP_TIME
source initialize_db.sh