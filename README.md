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
- [ ] `/fragment` PUT
- [X] `/fragment/:id` GET
- [X] `/fragment/:id` DELETE
- [X] `/fragment/:id/reorder` PUT
