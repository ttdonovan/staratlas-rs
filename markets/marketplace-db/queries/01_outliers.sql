DROP VIEW IF EXISTS v_galactic_iqr_bounds;

CREATE VIEW v_galactic_iqr_bounds AS
-- Calculate the IQR for the prices
WITH price_stats AS (
    SELECT
        cargo_type,
        order_side,
        PERCENTILE_CONT(0.25) WITHIN GROUP (ORDER BY price) AS Q1,
        PERCENTILE_CONT(0.75) WITHIN GROUP (ORDER BY price) AS Q3
    FROM
        orders AS o
    JOIN
        galactic_mints AS g
    ON
        o.asset_mint = g.asset_mint
    GROUP BY
        cargo_type,
        order_side
)

-- Determine the lower and upper bounds for outliers
, bounds AS (
    SELECT
        cargo_type,
        order_side,
        Q1,
        Q3,
        Q3 - Q1 AS IQR,
        CAST(Q1 - 1.5 * (Q3 - Q1) AS INT64) AS lower_bound,
        CAST(Q3 + 1.5 * (Q3 - Q1) AS INT64) AS upper_bound,
    FROM
        price_stats
)

SELECT
    cargo_type,
    order_side,
    Q1,
    Q3,
    IQR,
    lower_bound,
    upper_bound
FROM
    bounds;

-- Filter outlier 'Buy' orders where price is below the lower bound
DELETE FROM orders
WHERE account IN (
    SELECT o.account
    FROM orders AS o
    JOIN galactic_mints AS g
    ON o.asset_mint = g.asset_mint
    WHERE o.price < (SELECT lower_bound FROM v_galactic_iqr_bounds WHERE cargo_type = g.cargo_type AND order_side = 'Buy')
);

-- Filter outlier 'Sell' orders where price is above the upper bound
DELETE FROM orders
WHERE account IN (
    SELECT o.account
    FROM orders AS o
    JOIN galactic_mints AS g
    ON o.asset_mint = g.asset_mint
    WHERE o.price > (SELECT upper_bound FROM v_galactic_iqr_bounds WHERE cargo_type = g.cargo_type AND order_side = 'Sell')
);