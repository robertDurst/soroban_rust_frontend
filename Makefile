run:
	@if [ -z "$(file)" ]; then \
		echo "Usage: make run file=path/to/your/file.dtr"; \
  		exit 1; \
  fi; \
	./to_dtr $(file)

version_cmd := grep -m 1 '^version' Cargo.toml | awk -F ' = ' '{print $$2}' | tr -d '"'

.PHONY: version

version:
	@$(version_cmd)

setup:
	@echo "no setup needed"

release:
	rm -rf to_dtr && cargo build --release --bin to_dtr --target x86_64-unknown-linux-musl && mv target/x86_64-unknown-linux-musl/release/to_dtr .
