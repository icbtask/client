default:
    @just --list

test:
    @cargo nextest run

lint:
    @cargo clippy --workspace --all-features -- -D warnings
    @cargo fmt --all -- --check

build:
    @cargo build

update:
    @cargo upgrade
