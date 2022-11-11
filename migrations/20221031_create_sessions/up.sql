create table sessions (
  id serial primary key,
  rpc_session_keys jsonb unique not null
);