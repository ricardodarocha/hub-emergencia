CREATE TABLE public.users (
    id VARCHAR NOT NULL default gen_random_uuid()::VARCHAR,
    nome VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    APP_ID VARCHAR NULL,
    senha text NOT NULL,
    CONSTRAINT users_email_application_key UNIQUE (email, APP_ID),
    CREATED_AT TIMESTAMP not null default CURRENT_TIMESTAMP,
    CONSTRAINT users_pkey PRIMARY KEY (id)
);