# Prometeo CLI

This tool was developed during a hackathon promoted by the Ignite Community.

The challenge was to develop a CLI that could interact with the Prometeo Open Banking API.

## Features

#### Auth

- Login
- Log Out

#### Config

- Set API key

#### Meta

- List providers

#### Transactional Data

- Accounts
- Credit Cards

## Tech stack

#### Rust

##### Why?

Every developer should have at least one project with Rust otherwise you're irrelevant

##### Crates used:

- reqwest: makes requests
- tokio: asynchronous support for rust
- dialoguer: those nice menus you saw
- colored: adds colors to the displayed messages
- serde: for serialization
- serde_json: for json support
- jfs: abstraction to read and write json to the filesystem
- dirs: cross-platform directories (such as `~/.config` or `/home/${username}`) 

#### JSON

You can store things with it, gets the job done. Storing data such as  `api_key`, `user_key` and `username` seems convenient, although a safer approach might be preferable.

## Reference

[Ignite Community](https://joinignitecommunity.com/)

[Challenge](https://joinignitecommunity.com/desafio-cli/)