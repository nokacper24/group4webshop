#!/bin/sh
docker pull ghcr.io/nokacper24/group4webshop:main
docker pull ghcr.io/nokacper24/group4webshop:dev
docker compose down
docker compose up -d