# tairitsu-rs

a Discord bot for Arcaea, using the [poise](https://github.com/serenity-rs/poise) framework! the public instance is available [here](https://tairitsu.nazunacord.net)!

**note**: this project is **_not_** affiliated with lowiro in any way possible.

## usage

### bare metal

1. prepare a Postgres database.
2. make a copy of `.env.example` and fill in the data, like such:

```
DISCORD_TOKEN=your_bot_token
DATABASE_URL=postgres://your_username:your_password@your_database_url/your_schema
INVITE_LINK=optional_invite_url
```

3. if you cloned the repository, run `cargo run --release`. if you obtained the binary from the releases, execute said binary. this is to initialize the database.
4. fill in songs, charts and jackets data. an utility script to parse `songlist` and `packlist` is available. said files and other data (such as jackets) must be sourced by your own means.
5. enjoy!

### docker

a sample [docker-compose](docker-compose.yml) file is available. by default it also loads your `.env` file, but it can be modified to have environment variables passed directly into it.

you will still need to fill your own songs, charts and jackets data into the Postgres instance.

## license

licensed under either of

-   Apache License, Version 2.0  
    ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
-   MIT license  
    ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## contribution

unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
