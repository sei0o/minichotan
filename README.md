# minichotan

A simple frontend for [binchotan](https://github.com/sei0o/binchotan-backend), another Twitter (and Fediverse) client.   

## Installation

1. Clone this repository.
2. Copy and modify `.env.template` to fit to your environment.
3. run `cd frontend && npm install`.

## Starting the servers 

You have to run three servers: frontend (Svelte running on your browser), backend (Axum), and the reverse proxy (Caddy) for these two. 

- Run frontend: `$ just frontend`
- Run backend: `$ just backend`
- Run reverse proxy: `$ just proxy`

Open `$PROXY_LISTEN_ADDRESS` in your browser and you will see minichotan's interface.

Make sure that you launch [binchotan-backend](http://github.com/sei0o/binchotan-backend). "backend" points to the Axum-based API server or the binchotan-backend instance (RPC Server) depending on context. This is confusing but we just don't know how to name these servers.