default:
    just --list
build:
    cargo build --release
run:
    cargo run --release
install:
    cargo install --locked --path .
