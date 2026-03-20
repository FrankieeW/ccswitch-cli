.PHONY: test test-db test-docker test-local test-clean help

help:
	@echo "ccswitch-cli test targets:"
	@echo "  make test        - Run all tests with test database (local)"
	@echo "  make test-db     - Create/refresh test database"
	@echo "  make test-docker - Run tests in Docker"
	@echo "  make test-clean  - Clean test artifacts"

test: test-db
	./scripts/test-with-db.sh

test-db:
	@echo "Test database ready at test-fixtures/test.db"
	@ls -la test-fixtures/test.db 2>/dev/null || (echo "Creating test DB..." && ./scripts/create-test-db.sh)

test-docker:
	docker build -f Dockerfile.test -t ccswitch-test .
	docker run --rm -v "$$(pwd):/app" ccswitch-test cargo test --all-targets

test-local:
	CCSWITCH_DB_PATH=test-fixtures/test.db cargo test --all-targets

test-clean:
	rm -rf test-fixtures/*.db
	rm -rf target/
