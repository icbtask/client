default:
    @just --list

test:
    @cargo nextest run

build:
    @cargo build

update:
    @cargo update
