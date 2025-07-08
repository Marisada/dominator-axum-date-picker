# Date Picker Demo

Thai date picker demo in [Rust](https://www.rust-lang.org/) using [dominator](https://github.com/Pauan/rust-dominator) and [axum](https://github.com/tokio-rs/axum)

## Requirements
- Install Rust from [www.rust-lang.org](https://www.rust-lang.org/tools/install) or
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
- Add wasm32-unknown-unknown target
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
- Add ubuntu/debian build-essential
    ```bash
    sudo apt install build-essential
    ```
- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/) WASM packager
    ```bash
    cargo install wasm-pack
    ```
    or
    ```bash
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    ```
- Install [grass](https://github.com/connorskees/grass) SCSS compiler
    ```bash
    cargo install grass
    ```

## How to run
Linux
```sh
chmod +x *.sh
./css-bundle.sh
./client.sh
./serve.sh
```

Windows
```bat
css-bundle
client
serve
```

Inspired by [Seed DatePicker](https://github.com/tommket/seed-datepicker)
