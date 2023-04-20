# Deployment
We tried to make the deployment as easy and painless as possible.  

## Requirements
- [Docker Engine](https://docs.docker.com/engine/install/)  
- [Certbot](https://certbot.eff.org/instructions?ws=other&os=ubuntufocal) 
 
Install Docker Engine and follow Certbot instructions.

## Deployment
The process is to clone this repository, move to this directory, setup [`.env`](#setup-env-file) file and run [first_start.sh](first_start.sh) script.
```bash
git clone https://github.com/nokacper24/group4webshop.git
cd group4webshop/deployment

nano .env # setup .env file, see next section
./first_start.sh
```
This script will run the provided [docker-compose.yml](docker-compose.yml), create the database schema and add backend-user to the database.  

Congratulations, you have deployed the webshop!
### Setup .env file
Environmental variables are used to configure the deployment and secrets.  
This file should be places in the deployment directory and named `.env`.  
The file should contain the following variables:
```bash
DB_PASSWORD=password # database root password
BACKEND_USR_PASS=password # backend user password
IMG_TAG=main # which docker image tag should be used
```

**Note:** Cert path, port and other variables [required by Webshop Server](../webshop_server/README.md#environmental-variable-needed-to-run) are hardcoded in the [docker-compose.yml](docker-compose.yml) file, to simplify our deployment process.

### Updating
To update to the latest image, run the following script:
```bash
./update.img.sh
```
This will pull latest images and run `docker compose down` and `docker compose up -d` commands.