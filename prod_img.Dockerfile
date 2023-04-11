# syntax=docker/dockerfile:1

# Use an official Node runtime as a parent image
FROM node:lts AS react-build

# Set the working directory
WORKDIR /app

# Copy package.json and package-lock.json to the container
COPY webshop_frontend/package*.json ./

# Install dependencies
RUN npm install

# Copy the rest of the application code to the container
COPY webshop_frontend/ ./

# Build the React app
RUN npm run build

# Use an official Rust runtime as a parent image
FROM rust:latest AS rust-build


# Set the working directory
WORKDIR /app

# copy all rust files, including src, from webshop_server
COPY webshop_server/ ./

# rust expects the dist file in webshop_frontend/dist
COPY --from=react-build /app/dist ./webshop_frontend/dist/

# Set the environment variable for the frontend directory
ENV FRONT_DIST_DIR=webshop_frontend/dist

# Set env vat to use offline db
ENV SQLX_OFFLINE=true

# Build the Rust app
RUN cargo build --release

# Use an official Ubuntu runtime as a parent image
FROM alpine:latest

# Set the working directory
WORKDIR /app

# Copy built Rust app to the container
COPY --from=rust-build /app/target/release/webshop_server ./

# Start Rust server on container startup
RUN ls
CMD ["./webshop_server"]