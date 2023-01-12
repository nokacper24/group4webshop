## Running diesel  migrations
  - cargo install diesel_cli
  - echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
  - diesel setup
  - diesel migration generate {{NAME OF TABLE}}
