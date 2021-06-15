# Dominion

![GitHub](https://img.shields.io/github/license/tireswing/dominion)
[![dependency status](https://deps.rs/repo/github/tireswing/dominion/status.svg)](https://deps.rs/repo/github/tireswing/dominion)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/tireswing/dominion/CI)

STILL A WORK IN PROGRESS - NOT CURRENTLY USABLE

A backend library for playing Dominion, written entirely in Rust. Includes a sample command-line based text interface for running the game, however this library is designed with the expectation that the user will provide their own frontend via callbacks. This has two advantages: first, it allows anyone to create their own Dominion client, with any interface they want (for example, one client might be minimal and designed to run on as few resources as possible, while another could include more sophisticated graphics). Secondly, it enables building simulators to test out different Dominion strategies.

To see the documentation, clone the repository and then run

```shell
cargo doc --no-deps --open
```
