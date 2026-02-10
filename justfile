default: 
    just --list

fmt: 
    cargo fmt --all

lint:
    cargo clippy


build:
    cargo build --release

test: 
    cargo test

run: 
    cargo run 


