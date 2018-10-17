all:
	@echo "no default"

.PHONY: run
run:
	cargo run

.PHONY: test
test:
	cargo test
