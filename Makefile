.PHONY: all clippy fmt udeps check test sqlx-check coverage db
all: fmt clippy check test sqlx-check udeps

clippy:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt -- --check

udeps:
	cargo +nightly udeps

check:
	SQLX_OFFLINE=true cargo check

test:
	cargo test

sqlx-check:
	cargo sqlx prepare --workspace --check -- --all-targets

coverage:
	cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
	cargo llvm-cov report --html --output-dir coverage

db:
	./scripts/init_db.sh
