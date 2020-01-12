* Install `diesel_cli`
```shell script
cargo install diesel_cli --no-default-features --features postgres
```

* Reset development database (non-Docker way)
```postgresql
dropdb scarlett
createdb scarlett
```
