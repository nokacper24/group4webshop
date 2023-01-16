## Running diesel  migrations (for postgres)
  - cargo install diesel_cli --no-default-features --features "postgres"
  - echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
  - diesel setup
  - diesel migration generate {{NAME OF TABLE}}


  
  ### This might be hard to do on windows (because no pkgmanager), but it is possible.
