run:
	@if [ -z "$(file)" ]; then \
		echo "Usage: make run file=path/to/your/file.dtr"; \
  		exit 1; \
  fi; \
	./to_dtr $(file)

version:
	grep -m 1 '^version' Cargo.toml | sed -E 's/version = "(.*)"/\1/'