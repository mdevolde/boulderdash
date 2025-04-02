# syntax=docker/dockerfile:1

# Step 1 : Create an image for building the Rust project
FROM rust:1.85 AS builder

# Install the build dependencies
RUN apt-get update && apt-get install -y build-essential

# Install wasm-pack
RUN cargo install wasm-pack

# Create a new directory for the project
WORKDIR /usr/src/app

# Copy the project files
COPY . .

# Build the project with wasm-pack
RUN wasm-pack build --release --target web

# Step 2 : Create an image for running the Python server
FROM python:3.13-slim

# Install python dependencies
COPY requirements.txt /app/requirements.txt
RUN pip install --upgrade pip && pip install --no-cache-dir -r /app/requirements.txt

# Copy the Python file, templates and static files
COPY ./app.py /app/app.py
COPY ./templates /app/templates
COPY ./static /app/static

# Copy the built files from the previous image
COPY --from=builder /usr/src/app/pkg /app/static/out

# Define the working directory
WORKDIR /app

# Expose the port 8000
EXPOSE 8000

# Run the Python server
CMD ["python", "app.py"]
