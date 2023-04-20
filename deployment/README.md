# Deployment
We tried to make the deployment as easy and painless as possible.  

## Requirements
- [Docker Engine](https://docs.docker.com/engine/install/)  
- [Certbot](https://certbot.eff.org/instructions?ws=other&os=ubuntufocal) 
 
Install Docker Engine and follow Certbot instructions.

## Deployment
On the server, we need [Docker Engine](https://docs.docker.com/engine/install/) installed.  

The process then is to clone this repository, move to this directory, setup [`.env`](#setup-env-file) file and run [first_start.sh](first_start.sh) script.
```bash
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