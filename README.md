In this repo, there is a demonstratation of a microservice written in Rust, and connected to a MySQL database. It supports CRUD operations on a database table via a HTTP service interface. The microservice is compiled into WebAssembly (Wasm) and runs in the WasmEdge Runtime, which is a secure and lightweight alternative to natively compiled Rust apps in Linux containers. 

## Quickstart with Docker

The easiest way to get started is to use a version of Docker Desktop or Docker CLI with Wasm support.

* [Install Docker Desktop + Wasm (Beta)](https://docs.docker.com/desktop/wasm/)
* [Install Docker CLI + Wasm](https://github.com/chris-crone/wasm-day-na-22/tree/main/server)

Then, you just need to type one command.

```bash
docker compose up
```

This will build the Rust source code, run the Wasm server, and startup a MySQL backing database. It also starts a basic STATIC web interface (available at http://localhost:8090). See the [Dockerfile](Dockerfile) and [docker-compose.yml](docker-compose.yml) files. 
