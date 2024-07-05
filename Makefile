run:
	@if [ -z "$(file)" ]; then \
		echo "Usage: make run file=path/to/your/file.dtr"; \
  		exit 1; \
  fi; \
	./to_dtr $(file)

CURRENT_VERSION := $(grep -m 1 '^version' Cargo.toml | awk -F ' = ' '{print $2}' | tr -d '"')

version:
	echo $(CURRENT_VERSION)