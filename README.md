* Install `diesel_cli`
```shell script
cargo install diesel_cli --no-default-features --features postgres
```

* Refresh materialized view
```postgresql
REFRESH MATERIALIZED VIEW photo_ordering;
```
