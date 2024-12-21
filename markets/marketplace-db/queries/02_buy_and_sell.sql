DROP VIEW IF EXISTS v_galactic_buy_orders;
DROP VIEW IF EXISTS v_galactic_sell_orders;

CREATE VIEW v_galactic_buy_orders AS
SELECT
    o.account,
    o.order_initializer,
    g.cargo_type,
    o.price,
    o.price / 100000000 as atlas_per_unit,
    o.order_origination_qty,
    o.order_remaining_qty,
    o.price * o.order_remaining_qty / 100000000 as atlas_total_value,
    o.created_at,
FROM
    orders AS o
JOIN
    galactic_mints AS g
ON
    o.asset_mint = g.asset_mint
WHERE
    o.order_side = 'Buy';

CREATE VIEW v_galactic_sell_orders AS
SELECT
    o.account,
    o.order_initializer,
    g.cargo_type,
    o.price,
    o.price / 100000000 as atlas_per_unit,
    o.order_origination_qty,
    o.order_remaining_qty,
    o.price * o.order_remaining_qty / 100000000 as atlas_total_value,
    o.created_at,
FROM
    orders AS o
JOIN
    galactic_mints AS g
ON
    o.asset_mint = g.asset_mint
WHERE
    o.order_side = 'Sell';

DROP VIEW IF EXISTS v_ustur_buy_orders;
DROP VIEW IF EXISTS v_ustur_sell_orders;

CREATE VIEW v_ustur_buy_orders AS
SELECT
    o.account,
    o.order_initializer,
    c.x,
    c.y,
    c.cargo_type,
    o.price,
    o.price / 100000000 as atlas_per_unit,
    o.order_origination_qty,
    o.order_remaining_qty,
    o.price * o.order_remaining_qty / 100000000 as atlas_total_value,
    o.created_at,
FROM
    orders AS o
JOIN
    certificate_mints AS c
ON
    o.asset_mint = c.asset_mint
WHERE
    c.faction = 'Ustur'
AND
    o.order_side = 'Buy';

CREATE VIEW v_ustur_sell_orders AS
SELECT
    o.account,
    o.order_initializer,
    c.x,
    c.y,
    c.cargo_type,
    o.price,
    o.price / 100000000 as atlas_per_unit,
    o.order_origination_qty,
    o.order_remaining_qty,
    o.price * o.order_remaining_qty / 100000000 as atlas_total_value,
    o.created_at,
FROM
    orders AS o
JOIN
    certificate_mints AS c
ON
    o.asset_mint = c.asset_mint
WHERE
    c.faction = 'Ustur'
AND
    o.order_side = 'Sell';

DROP VIEW IF EXISTS v_mud_buy_orders;
DROP VIEW IF EXISTS v_mud_sell_orders;

CREATE VIEW v_mud_buy_orders AS
SELECT
    o.account,
    o.order_initializer,
    c.x,
    c.y,
    c.cargo_type,
    o.price,
    o.price / 100000000 as atlas_per_unit,
    o.order_origination_qty,
    o.order_remaining_qty,
    o.price * o.order_remaining_qty / 100000000 as atlas_total_value,
    o.created_at,
FROM
    orders AS o
JOIN
    certificate_mints AS c
ON
    o.asset_mint = c.asset_mint
WHERE
    c.faction = 'MUD'
AND
    o.order_side = 'Buy';

CREATE VIEW v_mud_sell_orders AS
SELECT
    o.account,
    o.order_initializer,
    c.x,
    c.y,
    c.cargo_type,
    o.price,
    o.price / 100000000 as atlas_per_unit,
    o.order_origination_qty,
    o.order_remaining_qty,
    o.price * o.order_remaining_qty / 100000000 as atlas_total_value,
    o.created_at,
FROM
    orders AS o
JOIN
    certificate_mints AS c
ON
    o.asset_mint = c.asset_mint
WHERE
    c.faction = 'MUD'
AND
    o.order_side = 'Sell';

DROP VIEW IF EXISTS v_oni_buy_orders;
DROP VIEW IF EXISTS v_oni_sell_orders;

CREATE VIEW v_oni_buy_orders AS
SELECT
    o.account,
    o.order_initializer,
    c.x,
    c.y,
    c.cargo_type,
    o.price,
    o.price / 100000000 as atlas_per_unit,
    o.order_origination_qty,
    o.order_remaining_qty,
    o.price * o.order_remaining_qty / 100000000 as atlas_total_value,
    o.created_at,
FROM
    orders AS o
JOIN
    certificate_mints AS c
ON
    o.asset_mint = c.asset_mint
WHERE
    c.faction = 'ONI'
AND
    o.order_side = 'Buy';

CREATE VIEW v_oni_sell_orders AS
SELECT
    o.account,
    o.order_initializer,
    c.x,
    c.y,
    c.cargo_type,
    o.price,
    o.price / 100000000 as atlas_per_unit,
    o.order_origination_qty,
    o.order_remaining_qty,
    o.price * o.order_remaining_qty / 100000000 as atlas_total_value,
    o.created_at,
FROM
    orders AS o
JOIN
    certificate_mints AS c
ON
    o.asset_mint = c.asset_mint
WHERE
    c.faction = 'ONI'
AND
    o.order_side = 'Sell';