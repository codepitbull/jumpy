# Jumpy
This is just an experiment using the (Quicksilver)[https://www.ryanisaacg.com/quicksilver/] library to learn [Rust](https://www.rust-lang.org/).

## Launching it
```cargo run```

## What it does
Not much. You can pan around the game world ...


## Graphics
Graphics are from the gread [Sticker Knight](https://ponywolf.itch.io/sticker-knight) and are available [here](https://github.com/coronalabs/Sticker-Knight-Platformer).

## Web Assembly 
Not yet working cause I had no time to finalize it.

Building the wasm file works, didn't get it to run imn the browser.

```
rustup target add wasm32-unknown-unknown
cargo install cargo-web

cargo-web build --release --target=wasm32-unknown-unknown

cargo web deploy
```
