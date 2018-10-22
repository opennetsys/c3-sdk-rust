all:
	@echo "no default"

.PHONY: run
run:
	cargo run

.PHONY: test
test:
	cargo test

.PHONY: start/example
start/example:
	@(cd example && cargo run)

.PHONY: test/payload
test/payload:
	@echo '["setItem","foo","bar"]' |  nc localhost 3333

.PHONY: publish
publish:
	@cargo publish
