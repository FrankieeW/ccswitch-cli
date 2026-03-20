.PHONY: test test-db test-docker test-local test-clean help

help:
	@echo "ccswitch-cli test targets:"
	@echo "  make test        - Run all tests with test database (local)"
	@echo "  make test-local  - Run tests directly with CCSWITCH_DB_PATH"
	@echo "  make test-docker - Run tests in Docker (requires Docker daemon)"
	@echo "  make test-clean  - Clean test artifacts"

test:
	./scripts/test-with-db.sh

test-db:
	@if [ -f test-fixtures/test.db ]; then \
		echo "Test database exists at test-fixtures/test.db"; \
	else \
		echo "ERROR: test-fixtures/test.db not found"; \
		echo "The test database must be created manually from the local DB"; \
		exit 1; \
	fi

test-docker:
	docker build -f Dockerfile.test -t ccswitch-test .
	docker run --rm -v "$$(pwd):/app" ccswitch-test cargo test --all-targets

test-local:
	CCSWITCH_DB_PATH=test-fixtures/test.db cargo test --all-targets

test-clean:
	rm -rf test-fixtures/*.db
	rm -rf target/
