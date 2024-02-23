# Notes

Wallet address: `2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77`

```
cargo run -p sa-lockers-cli -- find proxy 2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77
```

## Proxy Rewarder: Proxy (73mNZAQ7pqc8USH7LNRujDRpWQ7GLkxhqcyBMLZsiu7B)

Wallet address is the `owner` of the `Proxy` account.

```
Proxy {
    escrow: 83MpPJSzE4g8VCGSd5LdL7afZkGvM9gtKNvAFBFEBVhD,
    owner: 2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77,
    token_mint: poLisWXnNRwC6oBu1vHiuKQzFjGL4XDSu4g9qjz9qVk,
    bump: 255,
    proxy_token_account: EABuP7WsJq43peuZRwjyMZnZ4U1DLAuKJjbJiHGEHxLm,
}
```

## Proxy Rewarder: Proxy Escrow (83MpPJSzE4g8VCGSd5LdL7afZkGvM9gtKNvAFBFEBVhD)

Wallet address is the `escrow_owner` of the `ProxyEscrow` account.

```
ProxyEscrow {
    escrow_owner: 2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77,    
    bump: 251,
    amount: 0,
    escrow_started_at: 0,
    escrow_ends_at: 0,
    rewards_last_claimed_at: 0,
    amount_claimed: 0,
}
```

## Locked Voter: Escrow (GoNreRSzd1JiV9nbUEpXMwK8V53W2GxGeZHXLHAgS9eJ)

Proxy address is the `owner` of the `Escrow` account.

```
Escrow {
    locker: 5WmM9c9WE71y78Ah8Bp8vgyoStscM1ZZyaaFqRf8b2Qa,
    owner: 73mNZAQ7pqc8USH7LNRujDRpWQ7GLkxhqcyBMLZsiu7B,
    bump: 255,
    tokens: HFNopz7MKh2EK45e1ZuCc2AsDUTFf4H6U81SYUVSJa2z,
    amount: 600000000,
    escrowed_started_ts: 1708657715,
    escrowed_ends_ts: 1866308499,
    vote_delegate: 73mNZAQ7pqc8USH7LNRujDRpWQ7GLkxhqcyBMLZsiu7B,
}
```

## Locked Voter: Locker (5WmM9c9WE71y78Ah8Bp8vgyoStscM1ZZyaaFqRf8b2Qa)

```
Locker {
    base: daoTYPRczeC8zB7qF5AqWdxxpXKogr4M5h8B2aRJiW9,
    bump: 255,
    token_mint: poLisWXnNRwC6oBu1vHiuKQzFjGL4XDSu4g9qjz9qVk,
    locked_supply: 6847844634867165,
    governor: D5r5xy9whfRjM7BS5Sskz1Kiyf8CkvnYnK9VzuRgpYbK,
    params: LockerParams {
        whitelist_enabled: true,
        max_stake_vote_multiplier: 10,
        min_stake_duration: 1,
        max_stake_duration: 157680000,
        proposal_activation_min_votes: 20000000000000000,
    },
}
```
