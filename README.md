# Hive :honeybee: :beetle: :ant:
## What is this project?
This is a personal project for an online hive server. Think chess.com, only for the board game [hive](https://boardgamegeek.com/boardgame/2655/hive). The end-game of the project is to be able to play hive games with your friends and for friends to spectate games live :mag: :sunglasses:

## What is hive?
Hive is a board game where the objective of the game is to place down bugs :bug: in order to surround the enemy queen bee piece :honeybee:

## How to run
### In production
You can use docker :whale:!
However, you probably want to change some URLs in the project to your domain.
See `frontend/src/request.rs`.
If you need to change the port, you need to change it in the `docker-compose` file as well as in the code.

### Locally
#### Server
```bash
cargo run
```

#### Frontend
Dependencies:
* wasm-pack
* cargo-make

All can be installed with `cargo install <dep>`

To make and serve the frontend you use cargo make:
```bash
cargo make watch  # Wathces and builds the frontend
cargo make serve # Serve the built project
```

#### Database
```bash
docker-compose up db
```

## Tech stack

|  |  |
| --- | --- |
| Database | MongoDB :bookmark_tabs:|
| Backend| Rust using Tokio + hyper :tokyo_tower:|
| Frontend| Rust using Seed-rs (elm inspired architecture) :seedling:
