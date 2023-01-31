#!/bin/bash
docker-compose up -d
sleep 5
docker-compose exec db bash -c "psql -U postgres -c 'CREATE DATABASE proflex;'"
