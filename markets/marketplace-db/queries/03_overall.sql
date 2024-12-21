DROP VIEW IF EXISTS v_galactic_buy_stats;
DROP VIEW IF EXISTS v_galactic_sell_stats;

CREATE VIEW v_galactic_buy_stats AS
SELECT
    cargo_type,
    MIN(price) AS min_price,
    MAX(price) AS max_price,
    CAST(MEAN(price) AS INT64) AS mean_price,
    SUM(order_remaining_qty) AS total_remaining_qty,
    COUNT(*) AS count,
    min_price / 100000000 as atlas_min_price,
    max_price / 100000000 as atlas_max_price,
    mean_price / 100000000 as atlas_mean_price
FROM
    v_galactic_buy_orders
GROUP BY
    cargo_type;

CREATE VIEW v_galactic_sell_stats AS
SELECT
    cargo_type,
    MIN(price) AS min_price,
    MAX(price) AS max_price,
    CAST(MEAN(price) AS INT64) AS mean_price,
    SUM(order_remaining_qty) AS total_remaining_qty,
    COUNT(*) AS count,
    min_price / 100000000 as atlas_min_price,
    max_price / 100000000 as atlas_max_price,
    mean_price / 100000000 as atlas_mean_price
FROM
    v_galactic_sell_orders
GROUP BY
    cargo_type;

DROP VIEW IF EXISTS v_overall;
CREATE VIEW v_overall AS
-- Calculate the overall means for each cargo type
WITH overall_mean AS (
    SELECT
        c.cargo_type,
        o.order_side,
        MEAN(o.price) AS overall_mean_price,
        MEAN(o.order_remaining_qty) AS overall_mean_remaining_qty,
        SUM(o.order_remaining_qty) AS overall_sum_remaining_qty
    FROM
        orders AS o
    JOIN
        certificate_mints AS c
    ON
        o.asset_mint = c.asset_mint
    GROUP BY
        c.cargo_type,
        o.order_side
)

-- Calculate the means for each faction and cargo type, and join with the overall means
SELECT
    c.faction,
    c.cargo_type,
    o.order_side,
    COUNT(*) AS count,
    CAST(MIN(o.price) AS INT64) AS min_price,
    CAST(MAX(o.price) AS INT64) AS max_price,
    CAST(MEAN(o.price) AS INT64) AS mean_price,
    CAST(overall_mean.overall_mean_price AS INT64) as overall_mean_price,
    (MEAN(o.price) - overall_mean.overall_mean_price) / overall_mean.overall_mean_price * 100 AS percent_difference_price,
    CAST(MIN(o.order_remaining_qty) AS INT64) AS min_remaining_qty,
    CAST(MAX(o.order_remaining_qty) AS INT64) AS max_remaining_qty,
    CAST(MEAN(o.order_remaining_qty) AS INT64) AS mean_remaining_qty,
    CAST(overall_mean.overall_mean_remaining_qty AS INT64) as overall_mean_remaining_qty,
    (MEAN(o.order_remaining_qty) - overall_mean.overall_mean_remaining_qty) / overall_mean.overall_mean_remaining_qty * 100 AS percent_difference_remaining_qty,
    SUM(o.order_remaining_qty) AS sum_remaining_qty,
    overall_mean.overall_sum_remaining_qty,
FROM
    orders AS o
JOIN
    certificate_mints AS c
ON
    o.asset_mint = c.asset_mint
JOIN
    overall_mean
ON
    c.cargo_type = overall_mean.cargo_type
AND
    o.order_side = overall_mean.order_side
GROUP BY
    c.faction,
    c.cargo_type,
    o.order_side,
    overall_mean.overall_mean_price,
    overall_mean.overall_mean_remaining_qty,
    overall_mean.overall_sum_remaining_qty
ORDER BY
    count DESC;