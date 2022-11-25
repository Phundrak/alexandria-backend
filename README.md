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

## Docker
If you can use Docker, the easiest way to spin up Alexandria is to
execute the following command:
```shell
docker-compose build # build the Alexandria image
docker-compose up # spin up the project, add -d to detach
```

If you launched Alexandria as a detached container, you can stop it
with:
```shell
docker-compose down
```

## Unix
### Compile the project
```shell
cargo build
# or
cargo build --release
```

### Run the project
You will first need to set up the database. For that, install `diesel`:
```shell
cargo install diesel_cli
```

You can now run the migrations in order to prepare PostgreSQL. Note
that your PostgreSQL instance must be running.

```shell
diesel migration run
```

In case PostgreSQL isn’t already running, you can run it in Docker
with the provided `docker-compose.yml`. Just disable the Alexandria
container first (you can comment it out).

```shell
docker-compose up -d
# to turn it off:
docker-compose down
```

You can now run the server:
```shell
cargo run
# or
cargo run --release
```

### Lint
```shell
cargo clippy
```

## REST API
The specification of the REST API can be found in the file
`docs/api.yml`

### Currently implemented paths

#### Author
- [X] `/author` GET
- [X] `/author` POST
- [X] `/author` PUT
- [X] `/author/find` GET
- [X] `/author/:id` GET
- [X] `/author/:id` DELETE

#### Book
- [X] `/book` GET
- [X] `/book` POST
- [X] `/book` PUT
- [X] `/book/find` GET
- [X] `/book/:id` GET
- [X] `/book/:id` DELETE

#### Fragments
- [X] `/book/:id/fragments` GET
- [X] `/fragment` POST
- [X] `/fragment` PUT
- [X] `/fragment/:id` GET
- [X] `/fragment/:id` DELETE
- [X] `/fragment/:id/reorder` PUT
