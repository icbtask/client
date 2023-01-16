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

build-release-linux-x86_64-musl:
    podman run --rm -v .:/home/rust/src messense/rust-musl-cross:x86_64-musl cargo build --release
