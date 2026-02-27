# AUTHENTICATION API WITH RUST

```cmd
git clone
```
## Use this template and customize your own application


## Run the application
> (1) run postgre,
> for example run with docker
>```docker
> docker run --name rust-postgres 
> -e POSTGRES_PASSWORD=secret 
> -p 5432:5432 -d postgres:16
>```

> (2) create database for authentication data
>```sql
>psql create database database_name
>```

> (3) export your database url and other .env variables
>```cmd
>echo DATABASE_URL=postgres://postgres:secret@localhost:5432/database_name > .env
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
 
 
