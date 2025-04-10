<div align="center">

  <h1>HTTP Proxy Server</h1>

  <p>
    A lightweight Backend-for-Frontend (BFF) proxy server built in Rust, designed to proxy REST, GraphQL and tRPC requests.
  </p>

  <a href="https://github.com/iamanishx/proxy-xd/releases">
    <img src="https://img.shields.io/github/release/iamanishx/proxy-xd.svg" alt="Releases" />
  </a>
  <a href="https://hub.docker.com/r/iamanishx/proxy-xd">
    <img src="https://img.shields.io/docker/pulls/iamanishx/proxy-xd" alt="Docker Pulls" />
  </a>
  <a href="https://github.com/iamanishx/proxy-xd/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/iamanishx/proxy-xd" alt="License" />
  </a>

</div>

## 🔍 What is a BFF (Backend-for-Frontend)?

A **Backend-for-Frontend (BFF)** is a server that sits between your frontend application and backend services. It provides several advantages over  reverse proxies, specifically tailored to the frontend's needs.

---

### ✅ Problems Solved

- **CORS Issues**  
  Eliminates cross-origin resource sharing problems as this will work as backend to backend communication .

- **Performance**  
  Built in Rust for ultra-high throughput (100k+ requests/second).

- **Security**  
  Minimal attack surface via a tiny Alpine-based container (0 vulnerabilities).

---

## 📦 Image Details

- **Latest tag**: `iamanishx/proxy-xd:latest`  
- **Version tag**: `iamanishx/proxy-xd:0.1.0`  
- **Base image**: Alpine 3.19 (minimal footprint)  
- **Image size**: ~15MB  

---

## 🛠️ Development

This project is **open source** and contributions are welcome!

### 🔧 Build & Run Locally

bash

# Clone the repository
```
git clone https://github.com/iamanishx/proxy-xd.git
cd proxy-xd
```
# Build in release mode
```
cargo build --release
```

# Run the server
```
BACKEND_URL=http://localhost:4000/graphql 
cargo run
```

## 🚀 Quick Start

```bash
# Pull the Docker image
docker pull iamanishx/proxy-xd:latest

# Run the proxy (replace with your GraphQL/tRPC endpoint)
docker run -p 8080:8080 \
  -e BACKEND_URL=http://your-api-service:4000/graphql \
  -e LISTEN_ADDR=0.0.0.0:8080 \
  iamanishx/proxy-xd:latest
```
## 🔧 Configuration

| Environment Variable       | Description                           | Default         |
|---------------------------|---------------------------------------|-----------------|
| `BACKEND_URL`             | URL of your API service               | **Required**    |
| `LISTEN_ADDR`             | Address:port the proxy listens on     | `0.0.0.0:8080`  |
| `RUST_LOG`                | Logging level                         | `info`          |
| `MAX_IDLE_CONNS`          | Maximum idle connections              | `1000`          |
| `MAX_IDLE_CONNS_PER_HOST` | Maximum idle connections per host     | `1000`          |
 

🐳 Docker Compose Example
```
version: '3'
services:
  frontend:
    image: your-frontend-app
    ports:
      - "3000:3000"
    environment:
      - API_URL=http://proxy:8080

  proxy:
    image: iamanishx/proxy-xd:latest
    ports:
      - "8080:8080"
    environment:
      - BACKEND_URL=http://api:4000/graphql
      - LISTEN_ADDR=0.0.0.0:8080
      - RUST_LOG=info

  api:
    image: your-api-service
    ports:
      - "4000:4000"
```      

📄 License
MIT
