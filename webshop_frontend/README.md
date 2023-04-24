# Frontend readme
The frontend is a [React](https://react.dev/)  application, it is written in [TypeScript](https://www.typescriptlang.org/) due to its static typing. For bundling, we use [Vite](https://vitejs.dev/).  

## How to run
Before all, you will need to install [Node.js](https://nodejs.org/en/). Before building this project, you will also need all the dependencies, you can install them by running
```bash
npm install
```

Generally, the React application is built by Docker and included in the webshop image. However, if you'd like to build it and serve from backend locally, run
```bash
npm run build
```
This will build the application and put it in the `dist` folder, later when you build the backend, contents of `dist` will be copied to the backend.  

If you want to run the application in development mode, with live refresh, run
```bash
npm run dev
```
For this you will need to have the backend running, as well as environmental variables set, and SSL certificates in this directory. See next sections for more information.

## Environment variables
For development, you will need .env with the following variables:
```bash
VITE_URL="https://localhost"
VITE_PORT=8083 # or any other port you decide to run your backend on
```
These variables are used in the development server to proxy requests to the backend.

## SSL certificates
Since the backend only accepts HTTPS requests, frontend needs to be served over HTTPS as well. For this, you will need SSL certificates it this directory as well. In dev environment,  we recommand using self-signed certificates.