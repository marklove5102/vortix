#!/bin/bash

# Vortix Git Pre-commit Hook
# Mirrors CI checks to ensure only clean code is committed.

set -e

echo "🔍 Running pre-commit checks..."

# 1. Check Formatting
echo "🎨 Checking formatting (cargo fmt)..."
cargo fmt --all -- --check

# 2. Run Clippy
echo "📎 Running linter (cargo clippy)..."
cargo clippy -- -D warnings

# 3. Run Tests
echo "🧪 Running tests (cargo test)..."
cargo test

echo "✅ All checks passed! Proceeding with commit."
exit 0
