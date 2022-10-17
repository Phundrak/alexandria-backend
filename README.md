# Alexandria

> Backend manager for augmented books

## Project Setup

To get started, run the following command in the root of the project:

```shell
cp .env.example .env
```

Edit the new `.env` file to match your existing PostgreSQL instance.
Hopefully the whole project will soon be entirely available within a
Docker container.

### Compile the project
```shell
cargo build
# or
cargo build --release
```

### Run the project
```shell
cargo run --bin server
# or
cargo run --bin server --release
```

### Lint
```shell
cargo clippy
```

## REST API
The specification of the REST API can be found in the file `docs/api.yml`
