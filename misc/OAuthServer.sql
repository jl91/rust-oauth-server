CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE users(
  id bigserial NOT NULL,
  external_id uuid NOT NULL DEFAULT uuid_generate_v4(),
  username character varying NOT NULL,
  password_hash varchar(70) NOT NULL,
  is_active boolean NOT NULL,
  created_at timestamp without time zone NOT NULL DEFAULT now(),
  updated_at timestamp without time zone NOT NULL DEFAULT now(),
  deleted_at timestamp without time zone,
  CONSTRAINT users_pkey PRIMARY KEY(id)
);


CREATE UNIQUE INDEX idx_users_external_id ON users (external_id);

