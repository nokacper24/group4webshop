# Deployment
We tried to make the deployment as easy and painless as possible!   Therefore we use `Docker`, we build the image automatically with GithubActions on every push to [main](../../tree/main) or [dev](../../tree/dev) branch. Later on the server we use `docker compose` to run the image and setup the database. Additioanylly we made some scripts to further simplify the process.

## Pre-setup
Firstly, install [Docker Engine](https://docs.docker.com/engine/install/) and [Certbot](https://certbot.eff.org/) on the server. Then follow [Certbot instructions](https://certbot.eff.org/instructions?ws=other&os=ubuntufocal) in order to get the certificate necessary for HTTPS.

## Deployment
The deployment process is just a couple of steps. Firsly, clone this repository, then move to this directory, [setup `.env`](#setup-env-file) file and finally, run [first_start.sh](first_start.sh) script. You can use the following commands:
```bash
git clone https://github.com/nokacper24/group4webshop.git
cd group4webshop/deployment

nano .env # setup .env file, see next section
./first_start.sh
```
This script will run the provided [docker-compose.yml](docker-compose.yml), create the database schema and add backend-user to the database.  

Congratulations, you have deployed the webshop!
### Setup .env file
Environmental variables are used to configure the deployment and **secrets**.  
This file should be places in the deployment directory and named `.env`.  
The file should contain the following variables:
```bash
DB_PASSWORD=password # database root password
BACKEND_USR_PASS=password # backend user password
IMG_TAG=main # which docker image tag should be used, either main or dev
RUST_LOG=info,sqlx=warn # set the log level - optional, default is 'info,sqlx=warn
```

**Note:** Cert path, port and other variables [required by Webshop Server](../webshop_server/README.md#environmental-variable-needed-to-run) are hardcoded in the [docker-compose.yml](docker-compose.yml) file, to simplify our deployment process.

### Updating
To update to the latest image, run the following script:
```bash
./update.img.sh
```
This will pull latest images and run `docker compose up -d`, restarting only changed containers. 

We are looking into automating this process in the future.