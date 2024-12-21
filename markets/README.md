# Markets

## Articles

* https://chriseconomics.substack.com/p/unintended-consequences-game-economy
* https://aephia.com/star-atlas/sage-local-markets/
* https://aephia.com/star-atlas/sage-infrastructure-contracts/

## Marketplace Slint UI

A Galactic Marketplace application built with Slint UI.

```
$ cd markets/marketplace-slint-ui
$ cargo run
```

## Development

```
# dump all open marketplace orders as csv
$ cargo run -p sa-marketplace-cli -- dump-all-open path/to/output.csv
```

Explore data with DuckDB see [marketplace-db](marketplace-db/README.md).