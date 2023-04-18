# Webshop Server
This is the back-end for the webshop project. It is written in [Rust](https://www.rust-lang.org/), using the [Actix](https://actix.rs/) framework. We utilized [SQLx](https://crates.io/crates/sqlx) for database access and [PostgreSQL](https://www.postgresql.org/) for the database itself. For more information, please refer to the [webshop_server README](./webshop_server/README.md).

## How to compile and run
You will need to have Rust and Cargo installed. You can install them by following the instructions on [rustup.rs](https://rustup.rs/).  
In addition you will need a PostgreSQL database running. We recommend using Docker to run the database. You can install Docker by following the instructions on [docker.com](https://docs.docker.com/engine/).  

Since we're using [SQLx](https://crates.io/crates/sqlx), you *need* a live database in order to compile, because the sql queries are compile-time checked. If you however need to compile without a live database, [sqlx-data.json](./sqlx-data.json) file must be present. This file must be [regenerated](#regenerating-sqlx-datajson) if databse schema changes. You will also need to set `SQLX_OFFLINE` environment variable to `true`.

## Required environment variables


### Regenerating sqlx-data.json
[sqlx-data.json](./sqlx-data.json) must be regenerated if the database schema changes.  
YOu will need [SQLx CLI](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md), you can install it using cargo. Follow instructions on [SQLx CLI](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#install).  
In order to regenerate [sqlx-data.json](./sqlx-data.json), run the following command:
```bash
cargo sqlx prepare
```