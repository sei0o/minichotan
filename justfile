set dotenv-load

proxy:
  caddy run --watch

backend:
  cargo r

frontend:
  cd frontend; npm exec -- vite dev --host $FRONTEND_SERVER_HOST --port $FRONTEND_SERVER_PORT --strictPort