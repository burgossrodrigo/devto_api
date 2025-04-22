# DevTo API (Rust)

A simple Rust web service that fetches and serves your DEV.to blog posts via a local HTTP API.

## Table of Contents

- [About](#about)  
- [Tech Stack](#tech-stack)  
- [Prerequisites](#prerequisites)  
- [Installation](#installation)  
- [Usage](#usage)  
- [Project Structure](#project-structure)  
- [Docker](#docker)  
- [Contributing](#contributing)  
- [License](#license)  

## About

This service uses **Warp** as an HTTP server and **Reqwest** + **Serde** to fetch and deserialize your DEV.to articles from the `/api/articles/me/all` endpoint. It exposes a `GET /posts` endpoint that returns your posts as JSON.

## Tech Stack

- **Rust** – Programming language  
- **Warp** – High‑performance HTTP server and routing  
- **Reqwest** – HTTP client  
- **Serde** – JSON serialization / deserialization  
- **Docker** – Containerization  

## Prerequisites

- Rust toolchain (1.86.0 or later)  
- [cargo](https://doc.rust-lang.org/cargo/)  
- A DEV.to user API key (from https://dev.to/settings/account)  
- Docker (optional, for containerized builds)

## Installation

1. **Clone the repo**  
   ```bash
   git clone https://github.com/burgossrodrigo/devto_api.git
   cd devto_api

2. **Set your DEV.to API key**
Create a .env file in the project root:
```
API_KEY=your_devto_api_key_here
```

3. **Build and run locally**

```bash
cargo build --release
cargo run --release
```

## Usage

Once running, fetch your posts with:

```bash
curl http://127.0.0.1:3030/posts
```

Response example:

```json
{
  "articles": [
    {
      "id": 2393392,
      "title": "Access AWS ElastiCache from Localhost Using a Bastion Host and SSM",
      "description": "When managing cloud infrastructure...",
      "url": "https://dev.to/burgossrodrigo/...-5b60",
      "tag_list": ["redis","aws","devops"]
    },
    { /* ... */ }
  ]
}
```

## Project Structure

```bash
devto_api/
├── Cargo.toml                # dependencies & metadata
├── Dockerfile                # multi‑stage build for container
├── .env                      # environment variables (not committed)
└── src/
    ├── main.rs               # starts Warp server & routes
    ├── handler.rs            # Warp route handlers
    ├── use_case/
    │   └── api_call.rs       # fetching & processing DEV.to API
    └── domain/
        └── entities/
            └── post.rs       # Post struct (Serde model)

```

## Docker
Build the image

1. Build the image

```bash
docker build -t devto_api .
```

2. Run the container

```bash
docker run --env API_KEY=your_devto_api_key_here -p 3030:3030 devto_api
```
3. Test

```bash
curl http://localhost:3030/posts
```

## Contributing

Contributions are welcome!

1. Fork the repo
2. Create a feature branch:

```bash
git checkout -b feature/my-awesome-feature
```

3. Commit your changes
4. Push to your branch and open a Pull Request

Please follow Rust formatting conventions (cargo fmt) and add tests where applicable.

