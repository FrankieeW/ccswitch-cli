#!/bin/bash
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_DB="$PROJECT_ROOT/test-fixtures/test.db"

echo -e "${YELLOW}=== ccswitch-cli Test Runner ===${NC}"

if [ ! -f "$TEST_DB" ]; then
    echo -e "${RED}ERROR: Test database not found at $TEST_DB${NC}"
    exit 1
fi

TEMP_DIR=$(mktemp -d)
DB_DIR="$TEMP_DIR/.cc-switch"
mkdir -p "$DB_DIR"
cp "$TEST_DB" "$DB_DIR/cc-switch.db"

echo "Test database: $TEST_DB"
echo "Using CCSWITCH_DB_PATH=$DB_DIR/cc-switch.db"
echo ""

cd "$PROJECT_ROOT"
if [ -n "$1" ]; then
    echo -e "${YELLOW}Running test: $1${NC}"
    CCSWITCH_DB_PATH="$DB_DIR/cc-switch.db" cargo test "$1" --all-targets
else
    echo -e "${YELLOW}Running all tests${NC}"
    CCSWITCH_DB_PATH="$DB_DIR/cc-switch.db" cargo test --all-targets
fi

TEST_RESULT=$?
rm -rf "$TEMP_DIR"

if [ $TEST_RESULT -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed${NC}"
else
    echo -e "${RED}✗ Tests failed${NC}"
fi

exit $TEST_RESULT
