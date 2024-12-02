# Rust File Transfer API

This project is a file transfer API built with [Axum](https://github.com/tokio-rs/axum), designed to serve and list files from a specified directory. It provides basic authentication to ensure secure access to its endpoints.

## Features
- **File Listing**: Retrieve a list of files in a specified directory, including their sizes.
- **File Download**: Securely download files by their paths.
- **Basic Authentication**: Protects endpoints with a username/password combination.

## Requirements
- [Docker](https://www.docker.com/) and [Docker Compose](https://docs.docker.com/compose/)
- Rust (for local development)

## Getting Started

1. **Build and Run with Docker Compose**:
   ```bash
   docker-compose up --build

