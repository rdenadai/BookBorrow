# BookBorrow

This is just a small test program to put in practice my Rust learning.

It is a simple REST API which uses **Actix** and **SeaOrm** (and a bunch of other libs).

**ER Diagram**

![ER Diagram](https://raw.githubusercontent.com/rdenadai/BookBorrow/master/assets/er_diagram.png)

**API Endpoints**

![API Endpoints](https://raw.githubusercontent.com/rdenadai/BookBorrow/master/assets/api_endpoints.png)

# Deploy

Just run docker-compose:

```bash
$> docker-compose up --build
```

# Development

Install:

```bash
# Run migration by hand
$> cargo install sea-orm-cli
# Watch for changes on files when developing
$> cargo install cargo-watch
```

```bash
# Run initial migration
$> sea-orm-cli migrate up
# Execute the system
$> cargo watch -q -c -w src/ -x run
```
