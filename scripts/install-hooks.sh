#!/bin/bash

# Installation script for Vortix git hooks

HOOK_SRC="scripts/pre-commit.sh"
HOOK_DEST=".git/hooks/pre-commit"

if [ ! -f "$HOOK_SRC" ]; then
    echo "❌ Error: $HOOK_SRC not found. Run this from the project root."
    exit 1
fi

if [ ! -d ".git" ]; then
    echo "❌ Error: .git directory not found. Are you in the project root?"
    exit 1
fi

echo "⚙️  Installing pre-commit hook..."
cp "$HOOK_SRC" "$HOOK_DEST"
chmod +x "$HOOK_DEST"
chmod +x "$HOOK_SRC"

echo "✅ Pre-commit hook installed successfully!"
