#!/bin/bash
# Test runner script - runs tests with test database
# Usage: ./scripts/test-with-db.sh [test-name]

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_DB="$PROJECT_ROOT/test-fixtures/test.db"

echo -e "${YELLOW}=== ccswitch-cli Test Runner ===${NC}"
echo "Project root: $PROJECT_ROOT"
echo "Test database: $TEST_DB"

# Verify test database exists
if [ ! -f "$TEST_DB" ]; then
    echo -e "${RED}ERROR: Test database not found at $TEST_DB${NC}"
    echo "Run 'make test-db' first to create it"
    exit 1
fi

# Create a temp directory for test run
TEMP_DIR=$(mktemp -d)
cp "$TEST_DB" "$TEMP_DIR/cc-switch.db"
export HOME="$TEMP_DIR"
export CC_SWITCH_DB_DIR="$TEMP_DIR"

echo -e "${GREEN}Test environment prepared${NC}"
echo "Temp home: $TEMP_DIR"
echo ""

# Run tests
cd "$PROJECT_ROOT"
if [ -n "$1" ]; then
    echo -e "${YELLOW}Running test: $1${NC}"
    cargo test "$1" --all-targets
else
    echo -e "${YELLOW}Running all tests${NC}"
    cargo test --all-targets
fi

TEST_RESULT=$?

# Cleanup
rm -rf "$TEMP_DIR"

if [ $TEST_RESULT -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed${NC}"
else
    echo -e "${RED}✗ Tests failed${NC}"
fi

exit $TEST_RESULT
