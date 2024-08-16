# Crew

https://play.staratlas.com/crew

Crew Collection:
https://solscan.io/token/CREWSAACJTKHKhZi96pLRJXsxiGbdZaQHdFW9r7qGJkB

Crew Creator:
https://solscan.io/account/CrEWxSu2zBz4TwutmK3jRjQVP9FJQyXAFBmq3EqKaAFj

## Usage

```
$ cat tmp/crew.txt
4208
4433
5779

$ cat ./tmp/crew.txt | cargo run -p crew-utils --example galaxy > ./tmp/crew.csv
```

## Development

### DuckDB

https://duckdb.org/

Example usage with `tmp/crew.csv`:

```
summarize select * from 'tmp/crew.csv';

select * from 'tmp/crew.csv' where neuroticism > 0.8;

select id, rarity, name, faction, sex from 'tmp/crew.csv';
```