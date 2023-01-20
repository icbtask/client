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

build-release-linux:
    @cargo build --release --target=x86_64-unknown-linux-musl
    @strip target/x86_64-unknown-linux-musl/release/icbtask

build-release:
    @cargo build --release
