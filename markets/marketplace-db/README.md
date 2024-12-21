# Markeplace DB

Explore data with DuckDB.

```bash
$ duckdb
```

```sql
D .database
memory:

D IMPORT DATABASE 'db-init';
D INSERT INTO orders SELECT * FROM read_csv('path/to/output.csv');

-- remove noise/ignored orders
.read queries/00_remove_noise.sql

-- setup to remove order outliers
.read queries/01_outliers.sql

-- create views for all buy and sell orders (for each faction and galactic marketplace)
D .read queries/02_buy_and_sell.sql
D .tables
certificate_mints       v_galactic_iqr_bounds   v_oni_buy_orders      
galactic_mints          v_galactic_sell_orders  v_oni_sell_orders
orders                  v_galactic_sell_stats   v_overall
v_galactic_buy_orders   v_mud_buy_orders        v_ustur_buy_orders
v_galactic_buy_stats    v_mud_sell_orders       v_ustur_sell_orders

-- get all buy orders for 'Food' in 'Ustur' (using view)
D SELECT account, x, y, cargo_type, order_remaining_qty, atlas_per_unit, atlas_total_value FROM v_ustur_sell_orders WHERE cargo_type = 'Food' ORDER BY x, y, price;

-- all 'MUD' local marketplace orders (using table)
D SELECT * FROM orders AS o JOIN certificate_mints AS c ON o.asset_mint = c.asset_mint WHERE c.faction = 'MUD';

-- all galactic marketplace orders
D SELECT * FROM orders AS o JOIN galactic_mints AS g ON o.asset_mint = g.asset_mint;

-- depth 10 sell orders for 'Fuel' in 'Galactic' marketplace
D SELECT * FROM v_galactic_sell_orders WHERE cargo_type = 'Fuel' ORDER BY price ASC LIMIT 10;

-- depth 10 buy orders for 'Fuel' in 'Galactic' marketplace
D SELECT * FROM v_galactic_buy_orders WHERE cargo_type = 'Fuel' ORDER BY price DESC LIMIT 10;

-- get an overall picture of local marketplace orders
D .read queries/03_overall.sql
D DESC v_overall;
┌──────────────────────────────────┬─────────────┬─────────┬─────────┬─────────┬─────────┐
│           column_name            │ column_type │  null   │   key   │ default │  extra  │
│             varchar              │   varchar   │ varchar │ varchar │ varchar │ varchar │
├──────────────────────────────────┼─────────────┼─────────┼─────────┼─────────┼─────────┤
│ faction                          │ VARCHAR     │ YES     │         │         │         │
│ cargo_type                       │ VARCHAR     │ YES     │         │         │         │
│ order_side                       │ VARCHAR     │ YES     │         │         │         │
│ count                            │ BIGINT      │ YES     │         │         │         │
│ min_price                        │ BIGINT      │ YES     │         │         │         │
│ max_price                        │ BIGINT      │ YES     │         │         │         │
│ mean_price                       │ BIGINT      │ YES     │         │         │         │
│ overall_mean_price               │ BIGINT      │ YES     │         │         │         │
│ percent_difference_price         │ DOUBLE      │ YES     │         │         │         │
│ min_remaining_qty                │ BIGINT      │ YES     │         │         │         │
│ max_remaining_qty                │ BIGINT      │ YES     │         │         │         │
│ mean_remaining_qty               │ BIGINT      │ YES     │         │         │         │
│ overall_mean_remaining_qty       │ BIGINT      │ YES     │         │         │         │
│ percent_difference_remaining_qty │ DOUBLE      │ YES     │         │         │         │
│ sum_remaining_qty                │ HUGEINT     │ YES     │         │         │         │
│ overall_sum_remaining_qty        │ HUGEINT     │ YES     │         │         │         │
├──────────────────────────────────┴─────────────┴─────────┴─────────┴─────────┴─────────┤
│ 16 rows                                                                      6 columns │
└────────────────────────────────────────────────────────────────────────────────────────┘

D SELECT * FROM v_overall WHERE cargo_type = 'Fuel' ORDER BY faction, order_side;
```