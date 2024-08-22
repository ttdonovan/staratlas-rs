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
$ .read sql/00_setup.sql
$ .read sql/01_views.sql
$ export database 'tmp/my_crew.db';
```

## Resources

Star Atlas Crew: https://play.staratlas.com/crew
Crew Collection: https://solscan.io/token/CREWSAACJTKHKhZi96pLRJXsxiGbdZaQHdFW9r7qGJkB
Crew Creator: https://solscan.io/account/CrEWxSu2zBz4TwutmK3jRjQVP9FJQyXAFBmq3EqKaAFj