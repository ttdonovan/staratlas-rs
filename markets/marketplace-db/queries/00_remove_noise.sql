-- remove orders where asset mints that should be ignored, not needed for the 'resource' marketplace
DELETE FROM orders WHERE asset_mint NOT IN (SELECT asset_mint FROM galactic_mints UNION ALL SELECT asset_mint FROM certificate_mints);

-- remove 'stale' orders, e.g. orders that are older than 30 days ago
DELETE FROM orders
WHERE account IN (
    SELECT o.account
    FROM orders AS o
    JOIN galactic_mints AS g
    ON o.asset_mint = g.asset_mint
    WHERE o.created_at < CAST(EPOCH(CURRENT_TIMESTAMP - INTERVAL '30' DAY) AS INT64)
);

-- remove the 'noise' from the orders table, most likely these are 'fake' orders
DELETE FROM orders WHERE order_origination_qty = 1;

-- a 'price < 10000' (10_000 / 100_000_000 = 0.0001 atlas) unlike to be any 'genuine' (buy) order
DELETE FROM orders where price < 10000;