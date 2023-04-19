#!/bin/bash
# initialize_db.sh
# This script starts the database, creates the schema, and populates the database.
# It also creates a user called backend_user with the password provided as an argument and grants it access to the database.
#
# Options:
# -p|--password <password> - password for the backend_user

# parse arguments
while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        -p|--password)
        BACKEND_USR_PASS="$2"
        shift # past argument
        shift # past value
        ;;
        *)    # unknown option
        echo "Unknown option: $key
        Usage: initialize_db.sh [-p|--password <password>]
        Example: initialize_db.sh -p secure_password"
        exit 1
        ;;
    esac
done

# check if -p argument is provided
if [ -z "$BACKEND_USR_PASS" ]; then
    echo "Password for backend_user is mandatory.
    Please provide a value using -p|--password option.
    Example: ./initialize_db.sh -p secure_password"
    exit 1
fi

docker compose exec db bash -c "psql -U postgres -d proflex < /sql_scripts/initialize.sql"
docker compose exec db bash -c "psql -U postgres -d proflex < /sql_scripts/populate_prod.sql"
echo "CREATE USER backend_user WITH PASSWORD '$BACKEND_USR_PASS';" | docker compose exec -T db psql -U postgres -d proflex
echo "GRANT SELECT, UPDATE, INSERT, DELETE ON ALL TABLES IN SCHEMA public TO backend_user;" | docker compose exec -T db psql -U postgres -d proflex
echo "GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO backend_user;" | docker compose exec -T db psql -U postgres -d proflex
