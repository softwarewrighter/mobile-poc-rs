#!/bin/bash
# Quality check script for Rust Mobile POC
# Runs all quality gates: format, lint, test, build

set -e  # Exit on first error

echo "========================================="
echo "Running Quality Checks"
echo "========================================="
echo ""

# 1. Format check
echo "1. Checking code formatting with rustfmt..."
cargo fmt -- --check
echo "✓ Code is properly formatted"
echo ""

# 2. Lint check
echo "2. Running clippy linter..."
cargo clippy --all-targets --all-features -- -D warnings
echo "✓ No clippy warnings"
echo ""

# 3. Run tests
echo "3. Running tests..."
cargo test
echo "✓ All tests passed"
echo ""

# 4. Build check
echo "4. Building project..."
cargo build
echo "✓ Build successful"
echo ""

echo "========================================="
echo "All Quality Checks PASSED ✓"
echo "========================================="
