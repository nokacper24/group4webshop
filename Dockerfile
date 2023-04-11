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

# Copy the Cargo.toml and Cargo.lock files
COPY webshop_server/Cargo.toml webshop_server/Cargo.lock ./

# make dummy src/main.rs
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build the dependencies
RUN cargo build --release

# Remove dummy src/main.rs
RUN rm -r src

# Copy the rest of the application code to the container, src, build.rs and sqls-data.json
COPY webshop_server/src ./src
COPY webshop_server/build.rs webshop_server/sqlx-data.json ./

# Copy the frontend build to the container
COPY --from=react-build /app/dist ./webshop_frontend/dist/

# Set the environment variable for the frontend directory
ENV FRONT_DIST_DIR=webshop_frontend/dist

# Set the environment variable for the resources/images directory
ENV RESOURCES_DIR=/webshop/resources/images

# Set env var to use offline db
ENV SQLX_OFFLINE=true

# Build the Rust app
RUN cargo build --release

# Use an official rust runtime as a parent image
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy built Rust app to the container
COPY --from=rust-build /app/target/release/webshop_server ./

# Run the app
CMD ["./webshop_server"]