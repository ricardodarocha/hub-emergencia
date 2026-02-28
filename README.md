# HUB EMERGENCIA API

```cmd
git clone
```
API white label para apoiar ações dos projetos de voluntários e outras iniciativas para ajudar vítimas em situação de emergência, enchentes, resgate etc.

## Rodar o ambiente

Se você tiver rust instaldo so instalar as dependencias e configurar o banco de dados em seguida cargo run --release.
Se preferir uma opção precomilada acessar a pasta bin/windows/emgapi.exe

## Rodar o banco de dados

> (1) run postgre,
> for example run with docker
>```docker
> docker run --name rust-postgres 
> -e POSTGRES_PASSWORD=secret 
> -p 5432:5432 -d postgres:16
>```

> (2) create database for authentication data
>```sql
>psql -U postgres
>psql create database emergencia
>```

> (3) export your database url and other .env variables
>```cmd
>echo DATABASE_URL=postgres://postgres:postgres@localhost:5432/emergencia > .env
>echo PORT=3000 >> .env
>echo RUST_LOG=info,rust_rest_api=debug,tower_http=debug >> .env
>```

> (4) add the sqlx-cli tool to run migations
> ```rust
>cargo install sqlx-cli --no-default-features --features native-tls,postgres
>```

> (5) apply migrations, or if do you prefere, just apply migrations/*.sql to your database
>```rust
>sqlx migrate run
>```

> (6) run the application
>```rust
>cargo build --release
>cargo run --release

> (7) run the tests
> 
> Open test.http in vscode an run the requests POST CREATE USER, POST LOGIN, GET USER LIST
> 
> Install http extension to see some helper actions
 
 
