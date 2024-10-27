# Crew

A collection of scripts and utilities for managing Star Atlas crew data.

## Dependencies

* https://bun.sh
* https://duckdb.org
* https://rustup.rs

## Usage

### 1. Crew-Scripts (Typescript)

Collect a list of crew numbers:

```
$ cp .env.sample crew-scripts/.env
$ cd crew-scripts
# edit .env

$ bun install

$ bun run examples/00_metaplex.ts > ../tmp/crew.txt
- or -
$ bun run examples/00_metaplex.ts --owner 2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77 > ../tmp/crew.txt

$ cd ..
```

### 2. Crew-Utils (Rust)

Download crew data:

```
$ cp .env.sample .env
# edit .env
$ cargo run -p crew-utils --example 01_galaxy -- .tmp/crew.txt > ./tmp/crew.csv
```

### 3. Crew-Utils - CDN Crew (e.g. the all crew.json)

```
$ curl -o tmp/crew.json https://cdn.staratlas.com/crew.json

$ du -h tmp/crew.json
396M    tmp/crew.json

$ time cargo run --release -p crew-utils --example 02_cdn_crew tmp/crew.json tmp/crew.db
real    51m43.936s

$ du -h tmp/crew.db.parquet/crew.parquet
$ 28M     tmp/crew.db.parquet/crew.parquet
```

## Data Analysis

### Basic

Example usage with `tmp/crew.csv`:

```
summarize select * from 'tmp/crew.csv';

select * from 'tmp/crew.csv' where neuroticism > 0.8;

select id, rarity, name, faction, sex from 'tmp/crew.csv';
```

### Crew Relational Database

```
# see `sql/00_setup.sql` edit the path to `tmp/crew.csv`
$ duckdb
D .read sql/00_setup.sql
D .read sql/01_views.sql
D export database 'tmp/my_crew.db';
D .quit
```

```
$ duckdb
D import database 'tmp/my_crew.db';
D .tables
aptitude_gains      rarities            v_engineering_crew  v_operator_crew
aptitude_perks      skills              v_fitness_crew      v_science_crew
aptitudes           species             v_flight_crew
crew_members        universities        v_hospitality_crew
factions            v_command_crew      v_medical_crew
D describe crew_members;
D select * from crew_members;
D .quit
```

### CDN Crew

```
$ duckdb
D .open tmp/crew.db
D .tables
crew
D desc crew;
┌───────────────────┬──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┬─────────┬─────────┬─────────┬─────────┐
│    column_name    │                                                                   column_type                                                                    │  null   │   key   │ default │  extra  │
│      varchar      │                                                                     varchar                                                                      │ varchar │ varchar │ varchar │ varchar │
├───────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┼─────────┼─────────┼─────────┼─────────┤
│ mint_offset       │ INTEGER                                                                                                                                          │ YES     │         │         │         │
│ name              │ VARCHAR                                                                                                                                          │ YES     │         │         │         │
│ age               │ DOUBLE                                                                                                                                           │ YES     │         │         │         │
│ agreeableness     │ DOUBLE                                                                                                                                           │ YES     │         │         │         │
│ conscientiousness │ DOUBLE                                                                                                                                           │ YES     │         │         │         │
│ extraversion      │ DOUBLE                                                                                                                                           │ YES     │         │         │         │
│ neuroticism       │ DOUBLE                                                                                                                                           │ YES     │         │         │         │
│ openness          │ DOUBLE                                                                                                                                           │ YES     │         │         │         │
│ is_command        │ BOOLEAN                                                                                                                                          │ YES     │         │         │         │
│ is_flight         │ BOOLEAN                                                                                                                                          │ YES     │         │         │         │
│ is_engineering    │ BOOLEAN                                                                                                                                          │ YES     │         │         │         │
│ is_medical        │ BOOLEAN                                                                                                                                          │ YES     │         │         │         │
│ is_science        │ BOOLEAN                                                                                                                                          │ YES     │         │         │         │
│ is_operator       │ BOOLEAN                                                                                                                                          │ YES     │         │         │         │
│ is_hospitality    │ BOOLEAN                                                                                                                                          │ YES     │         │         │         │
│ is_fitness        │ BOOLEAN                                                                                                                                          │ YES     │         │         │         │
│ rarity            │ ENUM('anomaly', 'common', 'epic', 'legendary', 'rare', 'uncommon')                                                                               │ YES     │         │         │         │
│ faction           │ ENUM('mud', 'oni', 'unaligned', 'ustur')                                                                                                         │ YES     │         │         │         │
│ species           │ ENUM('high-punaab', 'human', 'mierese', 'photoli', 'profound-punaab', 'sogmian', 'tufa', 'ustur')                                                │ YES     │         │         │         │
│ sex               │ ENUM('body-1', 'body-2', 'female', 'male', 'n/a')                                                                                                │ YES     │         │         │         │
│ aptitudes         │ ENUM('command-anomalous', 'command-major', 'command-minor', 'engineering-anomalous', 'engineering-major', 'engineering-minor', 'fitness-anomal.  │ YES     │         │         │         │
│ aptitude_perks    │ ENUM('command', 'engineering', 'fitness', 'flight', 'hospitality', 'medical', 'operator', 'science')[]                                           │ YES     │         │         │         │
│ aptitude_gains    │ ENUM('anomalous', 'major', 'minor')[]                                                                                                            │ YES     │         │         │         │
├───────────────────┴──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┴─────────┴─────────┴─────────┴─────────┤
│ 23 rows                                                                                                                                                                                            6 columns │
└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
D select count(*) from crew;
┌──────────────┐
│ count_star() │
│    int64     │
├──────────────┤
│       693512 │
└──────────────┘
D select enum_range(null::rarity);
┌────────────────────────────────────────────────────┐
│          enum_range(CAST(NULL AS rarity))          │
│                     varchar[]                      │
├────────────────────────────────────────────────────┤
│ [anomaly, common, epic, legendary, rare, uncommon] │
└────────────────────────────────────────────────────┘
D select unnest(enum_range(null::aptitude));
┌────────────────────────────────────────────┐
│ unnest(enum_range(CAST(NULL AS aptitude))) │
│                  varchar                   │
├────────────────────────────────────────────┤
│ command-anomalous                          │
│ command-major                              │
│ command-minor                              │
│ engineering-anomalous                      │
│ engineering-major                          │
│ engineering-minor                          │
│ fitness-anomalous                          │
│ fitness-major                              │
│ fitness-minor                              │
│ flight-major                               │
│ flight-minor                               │
│ hospitality-anomalous                      │
│ hospitality-major                          │
│ hospitality-minor                          │
│ medical-major                              │
│ medical-minor                              │
│ operator-anomalous                         │
│ operator-major                             │
│ operator-minor                             │
│ science-anomalous                          │
│ science-major                              │
│ science-minor                              │
├────────────────────────────────────────────┤
│                  22 rows                   │
└────────────────────────────────────────────┘
D .mode table
D select mint_offset, rarity, name, age, agreeableness, conscientiousness, extraversion, neuroticism, openness, faction, species, sex, aptitudes from crew where list_contains(aptitudes, 'command-anomalous');
+-------------+---------+------------+------+--------------------+--------------------+--------------+-------------+--------------------+---------+---------+--------+-------------------------------------------------------+
| mint_offset | rarity  |    name    | age  |   agreeableness    | conscientiousness  | extraversion | neuroticism |      openness      | faction | species |  sex   |                       aptitudes                       |
+-------------+---------+------------+------+--------------------+--------------------+--------------+-------------+--------------------+---------+---------+--------+-------------------------------------------------------+
| 512679      | anomaly | Anna Tolle | 36.0 | 0.6500000000000001 | 0.6400000000000001 | 0.95         | 0.24        | 0.6600000000000001 | mud     | human   | female | [hospitality-major, medical-major, command-anomalous] |
| 20571       | anomaly | Anna Tolle | 36.0 | 0.79               | 0.1                | 0.96         | 0.52        | 0.5700000000000001 | mud     | human   | female | [command-anomalous, flight-major, fitness-major]      |
+-------------+---------+------------+------+--------------------+--------------------+--------------+-------------+--------------------+---------+---------+--------+-------------------------------------------------------+
D select mint_offset, rarity, name, age, agreeableness, conscientiousness, extraversion, neuroticism, openness, faction, species, sex, aptitudes from crew where name = 'Anna Tolle';
D ...
D .quit
```

## Resources

* Star Atlas Crew: https://play.staratlas.com/crew
* Crew Collection: https://solscan.io/token/CREWSAACJTKHKhZi96pLRJXsxiGbdZaQHdFW9r7qGJkB
* Crew Creator: https://solscan.io/account/CrEWxSu2zBz4TwutmK3jRjQVP9FJQyXAFBmq3EqKaAFj