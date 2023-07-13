lint:
    cargo clippy --fix --allow-staged
    cargo fmt

lint-pedantic:
    cargo clippy --fix --allow-staged -- -W clippy::pedantic
    cargo fmt

example EXAMPLE:
    cargo run --release --example {{EXAMPLE}}

