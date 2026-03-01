CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE public.app(
    UID VARCHAR primary key not null default gen_random_uuid()::VARCHAR,
    email varchar not null,
    nome varchar,
    responsavel varchar,
    repositorio varchar,
    tipo varchar not null DEFAULT 'private',
    url varchar not null,
    metadados json,
    CREATED_AT TIMESTAMP not null default CURRENT_TIMESTAMP
);

CREATE TABLE public.pontos(
    UID VARCHAR primary key not null default gen_random_uuid()::VARCHAR,
    NOME VARCHAR not NULL,
    categoria VARCHAR not null default 'Ponto de Coleta',
    tipo VARCHAR not null default 'Voluntario',
    -- Emergencial, Instituicao de Caridade, Igreja, Escola etc...
    MUNICIPIO varchar not null default 'Juiz de Fora',
    ENDERECO VARCHAR,
    telefone VARCHAR not null,
    BAIRRO VARCHAR not null,
    HORARIOS VARCHAR,
    RESPONSAVEL VARCHAR,
    APP_ID VARCHAR not null references APP(UID),
    PIX VARCHAR,
    CNPJ VARCHAR,
    itens varchar,
    gps varchar,
    metadados json,
    CREATED_AT TIMESTAMP not null default CURRENT_TIMESTAMP
);

create table public.votos_pontos(
    APP_ID VARCHAR not null references APP(UID),
    PONTO_ID VARCHAR not null references pontos,
    USER_ID VARCHAR not null,
    CREATED_AT TIMESTAMP not null default CURRENT_TIMESTAMP
);

create table public.ITENS_PONTOS (
    ITEM VARCHAR not null default 'AGUA',
    ACEITANDO BOOL,
    RECUSANDO BOOL,
    ESTOQUE INTEGER,
    APP_ID VARCHAR not null REFERENCES APP(UID),
    UPDATED_AT TIMESTAMP not null default CURRENT_TIMESTAMP
);

create table public.voluntarios(
    UID VARCHAR primary key not null default gen_random_uuid()::VARCHAR,
    APP_ID VARCHAR not null references APP(UID),
    NOME VARCHAR not NULL,
    telefone varchar not null,
    MUNICIPIO varchar not null default 'Juiz de Fora',
    categoria varchar,
    mensagem varchar not null,
    metadados json,
    CREATED_AT TIMESTAMP not null default CURRENT_TIMESTAMP
);

create table public.solicitacoes(
    UID VARCHAR primary key not null default gen_random_uuid()::VARCHAR,
    APP_ID VARCHAR not null references APP(UID),
    NOME VARCHAR not NULL,
    telefone varchar not null,
    MUNICIPIO varchar not null default 'Juiz de Fora',
    categoria varchar,
    metadados json,
    mensagem varchar not null,
    CREATED_AT TIMESTAMP not null default CURRENT_TIMESTAMP,
    status varchar not null default 'NOVO',
    votos integer
);