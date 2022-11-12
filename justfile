set dotenv-load

proxy:
  caddy run --watch

backend:
  cargo r

frontend:
  cd frontend; npm exec -- vite dev --port $FRONTEND_SERVER_PORT --strictPort