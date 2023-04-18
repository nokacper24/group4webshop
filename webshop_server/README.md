# Webshop Server
This is the back-end for the webshop project. It is written in [Rust](https://www.rust-lang.org/), using the [Actix](https://actix.rs/) framework. We utilized [SQLx](https://crates.io/crates/sqlx) for database access and [PostgreSQL](https://www.postgresql.org/) for the database itself. For more information, please refer to the [webshop_server README](./webshop_server/README.md).

## How to compile and run
First of all, you will need to have Rust and Cargo installed. You can install them by following the instructions on [rustup.rs](https://rustup.rs/).  
In addition you will need a PostgreSQL database running. We recommend using Docker to run the database. You can install Docker by following the instructions on [docker.com](https://docs.docker.com/get-docker/).  

Since we're using [SQLx](https://crates.io/crates/sqlx), you *need* a live database in order to compile, because the sql queries are compile-time checked. If you however need to compile without a live database, see [Build with no database](#build-with-no-database).

### Required environment variables
#### Build
```bash
# Database URL
DATABASE_URL=postgresql://user:password@url/db_name
# Or if you have no live database
SQLX_OFFLINE=true

# Optional, default is `resources/images` (relative path)
RESOURCES_DIR="/resources/images"
```
**Note:** `RESOURCES_DIR` env variable is useful if you need to specify an absolute path to the resources directory. We use it when building the Docker image. This variable **will not** be read from `.env` file, it must be set in the environment, for example with:  
Unix command: `export RESOURCES_DIR="/resources/images"`  
Windows powershell command: `$env:RESOURCES_DIR = "/resources/images"`

#### Run
```bash
DATABASE_URL=postgresql://user:password@url/db_name
HOST=localhost # optionbal, default 'localhost'
PORT=8080 # optional, default '8080'
ALLOWED_ORIGINS=https://group04.web-tek.ninja # list of allowed origins, separated `,`
CERT_PATH=certificate.pem # path to certificate
PRIV_KEY_PATH=privatekey.pem # path to privatekey
```
You can put all enviromental variables inside a `.env` file at the root of the rust project directory (this directory).  




### Build with no database
If you need to build the project without a live database, [`sqlx-data.json`](./sqlx-data.json) file must be present in the root directory of the rust project (this directory). This file must be [regenerated](#regenerating-sqlx-datajson) if the database schema or queries change. You will also need to set `SQLX_OFFLINE` environment variable to `true`. You can do this in the `.env` file, or by setting the environment variable in your shell.

### Regenerating sqlx-data.json
[`sqlx-data.json`](./sqlx-data.json) must be regenerated if the database schema or queries change.  
You will need [SQLx CLI](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md), you can install it using cargo. Follow instructions on [SQLx CLI](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#install).  
In order to regenerate [sqlx-data.json](./sqlx-data.json), run the following command:
```bash
cargo sqlx prepare
```