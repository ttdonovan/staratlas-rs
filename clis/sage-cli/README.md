# sage-cli

A simple command line interface for SAGE Labs.

## Usage

```
$ cargo run -p sa-sage-cli -- --help

Star Atlas: Sage CLI --> donations: 2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77 <--

Usage: sa-sage-cli.exe [OPTIONS] <COMMAND>

Commands:
  actions
  find
  show
  help     Print this message or the help of the given subcommand(s)

Options:
      --provider.cluster <CLUSTER>    RPC URL for the Solana cluster [env: PROVIDER_CLUSTER=https://solana-api.syndica.io/access-token/WPoEqWQ2auQQY1zHRNGJyRBkvfOLqw58FqYucdYtmy8q9Z84MBWwqtfVf8jKhcFh/rpc]
      --provider.wallet <WALLET>      Wallet keypair to use [env: PROVIDER_WALLET=tmp/id-hack-me.json]
      --sage.game_id <GAME_ID>        Sage Game's Pubkey [env: SAGE_GAME_ID=GameYNgVLn9kd8BQcbHm8jNMqJHWhcZ1YTNy6Pn3FXo5]
      --sage.profile_id <PROFILE_ID>  Sage Player Profile's Pubkey [env: SAGE_PROFILE_ID=8bAzn7Dcv4msX8wMcoaxjm5TvmDr9AKqN3QhQxGxSTjS]
  -h, --help                          Print help
```

Find all games:

```
$ cargo run -p sa-sage-cli -- find games

+----------------------------------------------+---------+--------------------------------------------------------------+
| Game ID                                      | Version | Mints                                                        |
+=======================================================================================================================+
| GameYNgVLn9kd8BQcbHm8jNMqJHWhcZ1YTNy6Pn3FXo5 | 0       | Mints {                                                      |
|                                              |         |     atlas: ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx,     |
|                                              |         |     polis: poLisWXnNRwC6oBu1vHiuKQzFjGL4XDSu4g9qjz9qVk,      |
|                                              |         |     ammo: ammoK8AkX2wnebQb35cDAZtTkvsXQbi82cGeTnUvvfK,       |
|                                              |         |     food: foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG,       |
|                                              |         |     fuel: fueL3hBZjLLLJHiFH9cqZoozTG3XQZ53diwFPwbzNim,       |
|                                              |         |     repair_kit: tooLsNYLiVqzg8o4m3L2Uetbn62mvMWRqkog6PQeYKL, |
|                                              |         | }                                                            |
+----------------------------------------------+---------+--------------------------------------------------------------+
```

Find player profile:

```
$ cargo run -p sa-sage-cli -- find player-profile

+----------------------------------------------+---------+----------------+---------------+
| Profile ID                                   | Version | Auth Key Count | Key Threshold |
+=========================================================================================+
| 8bAzn7Dcv4msX8wMcoaxjm5TvmDr9AKqN3QhQxGxSTjS | 0       | 1              | 1             |
+----------------------------------------------+---------+----------------+---------------+
```

Find fleets:

```
$ cargo run -p sa-sage-cli -- find fleet 'Hyena Fleet'

[
    (
        771Sgp2yb1h3XsCrQjFLRq5L74ZX6qD8wzbZmjGeMxtF,
        Fleet {
            version: 0,
            game_id: GameYNgVLn9kd8BQcbHm8jNMqJHWhcZ1YTNy6Pn3FXo5,
            owner_profile: 8bAzn7Dcv4msX8wMcoaxjm5TvmDr9AKqN3QhQxGxSTjS,
            fleet_ships: G4DfUwNcB13YWjW8Wn3oczt6CpqvA1dVqBbWYgACr89z,
            ship_counts: ShipCounts {
                total: 1,
                updated: 1,
                xx_small: 1,
                x_small: 0,
                small: 0,
                medium: 0,
                large: 0,
                capital: 0,
                commander: 0,
                titan: 0,
            },
            stats: ShipStats {
                movement_stats: MovementStats {
                    subwarp_speed: 8400,
                    warp_speed: 100000,
                    max_warp_distance: 175,
                    warp_cool_down: 60,
                    subwarp_fuel_consumption_rate: 515,
                    warp_fuel_consumption_rate: 1877,
                    planet_exit_fuel_amount: 5,
                },
                cargo_stats: CargoStats {
                    cargo_capacity: 249,
                    fuel_capacity: 450,
                    ammo_capacity: 104,
                    ammo_consumption_rate: 130,
                    food_consumption_rate: 140,
                    mining_rate: 2620,
                    upgrade_rate: 0,
                },
                misc_stats: MiscStats {
                    crew: 1,
                    respawn_time: 60,
                    scan_cool_down: 56,
                    scan_repair_kit_amount: 10,
                },
            },
            cargo_hold: FJHrZ8nyunrtuhJjt1ZHdX1gr5gHCZpSEguG6yuzNPAN,
            fuel_tank: EGD6dW4kDk4DL9UcfdSvCwxdRMnNuRXLuK8KYNyC12cd,
            ammo_bank: DwqURNCJ8G2MUYBRZMcUCpD3tp8zKR3EHFiMfKXpx7ZY,
            update_id: 4,
        },
    ),
]
```

Actions: Stop Mining

```
$ cargo run -p sa-sage-cli -- actions stop-mining 771Sgp2yb1h3XsCrQjFLRq5L74ZX6qD8wzbZmjGeMxtF

3cyMyxNqEEMA8caNrkqsNuHDs9a14otpXQ3hcd6V4FXUM4G9Q56Ppk4LGZzKoFk7jHtAMDy1x1u3FRLteHDkQ3x3
3XiindVfQokZy6JADbARC6muog6oEZvPPSb7Kdn4pBGFfLg6SQxaHzJ5HYgZ3nkz8GofoWPUJ6HJHJhGBEVSSk4J
```