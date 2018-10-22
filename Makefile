all:
	@echo "no default"

.PHONY: run
run:
	cargo run

.PHONY: test
test:
	cargo test

.PHONY: test/payload
test/payload:
	@echo '["setItem","foo","bar"]' |  nc localhost 3333
