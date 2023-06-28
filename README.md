# Blackout - A simple, fast, and lightweight CLI copilot for your daily tasks.

Blackout is a simple, fast, and lightweight CLI copilot for your daily tasks. It's written in Rust and uses [PawanOsman](https://github.com/PawanOsman/ChatGPT#self-host-your-own-api) API as Large Language Model services.

  <p align="center">
      <img src="/img/black-out.jpg" alt="blackout" height="400">
  </p>

## Installation

Before using it, you need to generate an API key from [PawanOsman discord server](https://discord.pawan.krd/) at `#bot` channel by typing `/key`. By default PawnOsman allowing you to consume the API only with 1 ip address. So if you want to change your IP address, run the following command:

```bash
blackout --reset-ip
```

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [PawanOsman API key](https://discord.pawan.krd/)

## Run using Cargo

When you have installed **Rust and generated an API key**, you can run Blackout using Cargo:

```bash
$ cargo run -- --help
```

## Build using Cargo

When you have installed **Rust and generated an API key**, you can run start building Blackout using Cargo:

```bash
$ cargo build --release
```

Moove the binary to `/usr/local/bin`:
```bash
$ sudo mv target/release/blackout /usr/local/bin/blackout
```

## Usage

To get help about the commands, you can use the `--help` flag:

```bash
$ blackout --help
```

To get help about a specific command, you can use the `-t` for the technology and `-a` for action you want to get help about.
For example, to get help about `docker` technology and `delete all volumes` action, you can use:

```bash
$ blackout -t docker -a 'delete all volumes'`
```

output should be like this:

```bash
─────────────────────────────────────────
 docker volume rm $(docker volume ls -q)
─────────────────────────────────────────
```
