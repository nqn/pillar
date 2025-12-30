#!/bin/sh
# Setup script for git hooks

HOOKS_DIR="$(cd "$(dirname "$0")" && pwd)"
GIT_HOOKS_DIR="$(git rev-parse --git-dir)/hooks"

echo "Setting up git hooks..."

# Copy or symlink pre-commit hook
if [ -f "$HOOKS_DIR/pre-commit" ]; then
    ln -sf "$HOOKS_DIR/pre-commit" "$GIT_HOOKS_DIR/pre-commit"
    echo "✓ Installed pre-commit hook"
else
    echo "❌ pre-commit hook not found"
    exit 1
fi

echo "✓ Git hooks setup complete!"
echo ""
echo "The pre-commit hook will:"
echo "  - Check code formatting with cargo fmt"
echo "  - Auto-format if needed and prompt you to review"
echo ""
echo "To bypass the hook (not recommended):"
echo "  git commit --no-verify"
