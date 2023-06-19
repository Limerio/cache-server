# Cache Server

Hello everyone

## Why I do this project ?

I want to present you a project of cache server like [redis](https://redis.io) or [memcached](https://memcached.org) with more externals features (I will present you in more details after) for my personal knowledge to understand rust and this core functionality through multiply projects (or crates 👀).
I have already make projects in rust like a [file hasher](https://github.com/Limerio/file-hash) (cli), a [webscraper](https://github.com/Limerio/webscraper) or a [currency converter](https://github.com/Limerio/currency-converter) (cli) but there are small rather than this project.

## What can this project contain ?

This project contains or will contain :

- [ ] Server
  - [x] Cli
    - [x] port definition (default: 8080)
    - [ ] password authentification
    - [ ] Config file (yaml, yml, json, conf)
  - [x] SET
  - [x] GET
  - [x] DEL
  - [x] ALL
  - [ ] COUNT
  - [ ] EXISTS
  - [ ] RENAME
  - [ ] FLUSH
  - [x] PING
- [ ] Client
  - [ ] API
    - [ ] SET
    - [ ] GET
    - [ ] DEL
    - [ ] ALL
    - [ ] COUNT
    - [ ] EXISTS
    - [ ] RENAME
    - [ ] FLUSH
    - [ ] PING
      - [ ] one time
  - [ ] APP
    - [ ] SET
    - [ ] GET
    - [ ] DEL
    - [ ] COUNT
    - [ ] RENAME
    - [ ] FLUSH
    - [ ] ALL
  - [x] Cli
    - [x] port definition (default: 8080)
    - [ ] password authentification
    - [ ] Config file (yaml, yml, json, conf)
    - [x] SET
    - [x] GET
    - [x] DEL
    - [x] ALL
    - [ ] COUNT
    - [ ] RENAME
    - [ ] EXISTS
    - [ ] FLUSH
    - [x] PING
      - [x] infinite
        - [ ] with interval
        - [x] without interval
      - [x] one time
  - [ ] Lib
    - [x] SET
    - [x] GET
    - [x] DEL
    - [x] ALL
    - [ ] COUNT
    - [ ] EXISTS
    - [ ] RENAME
    - [ ] FLUSH
    - [ ] PING
      - [ ] infinite
        - [ ] with interval
        - [ ] without interval
      - [ ] one time
