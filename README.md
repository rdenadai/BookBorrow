# BookBorrow

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
# Execute the system
$> cargo watch -q -c -w src/ -x run
```
