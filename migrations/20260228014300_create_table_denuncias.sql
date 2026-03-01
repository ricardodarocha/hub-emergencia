DROP TABLE IF EXISTS public.denuncias;

CREATE TABLE public.denuncias(
    UID VARCHAR primary key not null default gen_random_uuid()::VARCHAR,
    tipo varchar not null default 'GOLPE', --GOLPE DO PIX, PAGINA FAKE, OUTRO
    app_id varchar not null references app(uid),
    memorando varchar,
    url varchar,
    chave_pix varchar,
    evidencias varchar, --somente url
    metadados json,
    created_by varchar references users(id),
    CREATED_AT TIMESTAMP not null default CURRENT_TIMESTAMP
);