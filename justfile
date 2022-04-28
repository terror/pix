default:
  just --list

all: build test clippy fmt-check

build:
  cargo build

check:
 cargo check

clippy:
  cargo clippy --all-targets --all-features

dev-deps:
	cargo install trunk

fmt:
  cargo +nightly fmt

fmt-check:
  cargo +nightly fmt --all -- --check
  @echo formatting check done

run *args:
  cargo run -- --{{args}}

test:
  wasm-pack test --node

serve:
  trunk serve

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"
