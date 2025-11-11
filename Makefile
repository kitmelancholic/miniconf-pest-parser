CARGO ?= cargo

.PHONY: fmt clippy test doc check publish

fmt:
	$(CARGO) fmt

clippy:
	$(CARGO) clippy --all-targets -- -D warnings

test:
	$(CARGO) test --all

doc:
	$(CARGO) doc --no-deps

check: fmt clippy test doc

publish:
	$(CARGO) publish --dry-run
