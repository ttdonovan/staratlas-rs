## SETUP

Create a persisted database using the `duckdb` CLI.

```
$ duckdb marketplace.db
```

## Galactic Mints

```sql
DESC SELECT * FROM read_json('../../galaxy/galaxy.json');

DESC SELECT unnest(attributes) FROM read_json('../../galaxy/galaxy.json');

CREATE TABLE tmp_mints AS SELECT mint, name, unnest(attributes) FROM read_json('../../galaxy/galaxy.json');

CREATE TABLE galactic_mints AS SELECT mint AS asset_mint, name AS cargo_type FROM tmp_mints WHERE category = 'resource';

DROP TABLE tmp_mints;
```

## Certificate Mints

```sql
CREATE TABLE tmp_mints AS SELECT * FROM read_json('../certificateMints.json');

DESC tmp_mints;

┌─────────────┬─────────────┬─────────┬─────────┬─────────┬─────────┐
│ column_name │ column_type │  null   │   key   │ default │  extra  │
│   varchar   │   varchar   │ varchar │ varchar │ varchar │ varchar │
├─────────────┼─────────────┼─────────┼─────────┼─────────┼─────────┤
│ mint        │ VARCHAR     │ YES     │         │         │         │
│ starbase    │ VARCHAR     │ YES     │         │         │         │
│ cargoType   │ VARCHAR     │ YES     │         │         │         │
└─────────────┴─────────────┴─────────┴─────────┴─────────┴─────────┘

CREATE TABLE certificate_mints AS
SELECT
    REGEXP_EXTRACT(starbase, '^(.*) Starbase', 1) AS faction,
    REGEXP_EXTRACT(starbase, 'x: (-?\d+)', 1) AS x,
    REGEXP_EXTRACT(starbase, 'y: (-?\d+)', 1) AS y,
    mint AS asset_mint,
    cargoType AS cargo_type
FROM
    tmp_mints;

DROP TABLE tmp_mints;
```

## Orders

```sql
CREATE TABLE orders AS SELECT * FROM read_csv('path/to/output.csv');

DESC orders;

┌───────────────────────┬─────────────┬─────────┬─────────┬─────────┬─────────┐
│      column_name      │ column_type │  null   │   key   │ default │  extra  │
│        varchar        │   varchar   │ varchar │ varchar │ varchar │ varchar │
├───────────────────────┼─────────────┼─────────┼─────────┼─────────┼─────────┤
│ account               │ VARCHAR     │ YES     │         │         │         │
│ order_initializer     │ VARCHAR     │ YES     │         │         │         │
│ asset_mint            │ VARCHAR     │ YES     │         │         │         │
│ order_side            │ VARCHAR     │ YES     │         │         │         │
│ price                 │ BIGINT      │ YES     │         │         │         │
│ order_origination_qty │ BIGINT      │ YES     │         │         │         │
│ order_remaining_qty   │ BIGINT      │ YES     │         │         │         │
│ created_at            │ BIGINT      │ YES     │         │         │         │
└───────────────────────┴─────────────┴─────────┴─────────┴─────────┴─────────┘
```

## Export Database

```sql
-- DELETE FROM orders;
EXPORT DATABASE 'db-init';
```