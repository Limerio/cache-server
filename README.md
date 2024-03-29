# Cache Server

Hello everyone

## Why I do this project ?

I want to present you a project of cache server like [redis](https://redis.io) or [memcached](https://memcached.org) with more externals features (I will present you in more details after) for my personal knowledge to understand rust and this core functionality through multiply projects (or crates 👀).

I have already make projects in rust like a [file hasher](https://github.com/Limerio/file-hash) (cli), a [github reposlist webscraper](https://github.com/Limerio/github-reposlist-webscraper) or a [currency converter](https://github.com/Limerio/currency-converter) (cli) but there are small rather than this project.

## What can this project contain ?

This project contains or will contain :

- [ ] Server
  - [ ] Cli
    - [x] port definition (default: 8080)
    - [ ] password authentification
    - [x] Config file (yaml, yml, json, toml)
  - [x] SET
  - [x] GET
  - [x] DEL
  - [x] ALL
  - [x] COUNT
  - [x] EXISTS
  - [x] RENAME
  - [x] FLUSH
  - [x] PING
- [ ] Client
  - [ ] API
    - [ ] Cli
      - [x] API
        - [x] port definition (default: 8000)
      - [ ] DB
        - [x] port definition (default: 8080)
        - [ ] password authentification
      - [x] Config file for all (yaml, yml, json, toml)
    - [x] SET
    - [x] GET
    - [x] DEL
    - [x] ALL
    - [x] COUNT
    - [x] EXISTS
    - [x] RENAME
    - [x] FLUSH
    - [x] PING
  - [ ] APP
    - [ ] SET
    - [ ] GET
    - [ ] DEL
    - [ ] COUNT
    - [ ] RENAME
    - [ ] FLUSH
    - [ ] ALL
  - [ ] Cli
    - [x] port definition (default: 8080)
    - [ ] password authentification
    - [x] Config file (yaml, yml, json, toml)
    - [x] SET
    - [x] GET
    - [x] DEL
    - [x] ALL
    - [x] COUNT
    - [x] RENAME
    - [x] EXISTS
    - [x] FLUSH
    - [x] PING
      - [x] infinite
        - [x] with interval
        - [x] without interval
      - [x] one time
  - [x] Shared
    - [x] SET
    - [x] GET
    - [x] DEL
    - [x] ALL
    - [x] COUNT
    - [x] EXISTS
    - [x] RENAME
    - [x] FLUSH
    - [x] PING
