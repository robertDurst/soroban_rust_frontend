run:
	@if [ -z "$(file)" ]; then \
		echo "Usage: make run file=path/to/your/file.dtr"; \
  		exit 1; \
  fi; \
	./to_dtr $(file)

version:
	result=$(grep -m 1 '^version' Cargo.toml | awk -F ' = ' '{print $2}' | tr -d '"')
	echo "$result"