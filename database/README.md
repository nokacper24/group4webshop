# Database
In order to run the project, you will need a PostgreSQL database. This directory contains a '[docker-compose.yml](docker-compose.yaml)' file that can be used to start a PostgreSQL database and adminer in your dev environment.

## Initial setup
Firstly, you will need to start the database. You can use the shell script [run_compose_up.sh](run_compose_up.sh) (Unix) or batch script [run_compose_up.bat](run_compose_up.bat) (Windows) to run the docker-compose file. Alternatively, you can run the following command from the context of this directory:
```bash
docker-compose up -d # Usually on Windows
# OR
docker compose up -d # Usually on linux
```
This will start the database and adminer. Adminer is a web-based database management tool that we use in the development enviroment for debugging and sanity checks. You can access it at [http://localhost:8080](http://localhost:8080). The default credentials are:
```
- System: PostgreSQL
- Username: postgres
- Password: password
```

## Creating the database schema
The database schema is created by running the SQL queries in the [./sql_scripts/init.sql](./sql_scripts/init.sql) file. You can use the shell script [./unix/initialize.sh](./unix/initialize.sh) (Unix) or batch script [./windows/initialize.bat](./windows/initialize.bat) (Windows) to run the queries.

## Populating the database
Again, this is done bu runnig the SQL scripts. There are two populate variants:
- [./sql_scripts/populate_prod.sql](./sql_scripts/populate_prod.sql) - Minimal data, only ProFlex company itslef and a admin user. Also a user for backend to use.
- [./sql_scripts/populate_dev.sql](./sql_scripts/populate_dev.sql) - More dummy data, nice for development.

## All good to go
Now you have a database running and the schema created. You can now return to the [webshop_server README](../webshop_server/README.md) and continue with the setup.

## Additional info
If you need to reset the database to its initial state, you can use the shell script [./unix/reset.sh](./unix/reset.sh) (Unix) or batch script [./windows/reset.bat](./windows/reset.bat) (Windows) to do so. This will remove the proflex databse and reinitialize it. It will not run the populate scripts, you have to do that manually.