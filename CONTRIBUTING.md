# Contributing to Pillar

Thank you for your interest in contributing to Pillar! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it before contributing.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce** the behavior
- **Expected behavior**
- **Actual behavior**
- **Environment details** (OS, Rust version, Pillar version)
- **Screenshots or terminal output** if applicable

### Suggesting Features

Feature suggestions are welcome! Please:

- **Check existing feature requests** to avoid duplicates
- **Provide a clear use case** for the feature
- **Explain how it would benefit users**
- **Consider implementation complexity**

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Make your changes** following our coding standards
3. **Add tests** for new functionality
4. **Update documentation** if needed
5. **Ensure all tests pass**
6. **Submit a pull request**

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git

### Getting Started

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/pillar.git
cd pillar

# Set up git hooks (automatic formatting checks)
./hooks/setup.sh

# Create a branch for your work
git checkout -b feature/my-feature

# Build the project
cargo build

# Run tests (use single-threaded execution to avoid race conditions)
cargo test -- --test-threads=1

# Run with your changes
cargo run -- init
```

### Project Structure

```
pillar/
├── src/
│   ├── main.rs          # Entry point
│   ├── cli.rs           # CLI argument parsing
│   ├── commands/        # Command implementations
│   │   ├── init.rs
│   │   ├── project.rs
│   │   ├── milestone.rs
│   │   ├── issue.rs
│   │   ├── comment.rs
│   │   └── view.rs
│   ├── models.rs        # Data structures
│   ├── parser.rs        # Markdown/YAML parsing
│   └── fs.rs            # File system operations
├── tests/               # Integration tests
└── plans/               # Design documents
```

## Coding Standards

### Git Hooks

The repository includes a pre-commit hook that automatically checks code formatting:

- Run `./hooks/setup.sh` to install the hooks
- The hook runs `cargo fmt --check` before each commit
- If formatting is needed, it will auto-format and ask you to review
- To bypass (not recommended): `git commit --no-verify`

### Rust Style

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Run `cargo fmt` before committing (automated by pre-commit hook)
- Run `cargo clippy` and address warnings
- Keep functions small and focused
- Use meaningful variable names
- Add doc comments for public APIs

### Code Example

```rust
/// Create a new project with the given name and priority
///
/// # Arguments
///
/// * `name` - The project name
/// * `priority` - Priority level (low, medium, high, urgent)
///
/// # Errors
///
/// Returns an error if the project already exists or if
/// file system operations fail
pub fn create_project(name: &str, priority: &str) -> Result<()> {
    // Implementation
}
```

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(comments): add comment system for issues

Implements the ability to add timestamped comments to issues,
milestones, and projects.

Closes #42
```

```
fix(issue): handle sanitized filenames correctly

Issue files use sanitized filenames (lowercase, no special chars)
but the lookup was case-sensitive.

Fixes #56
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_create_project

# Run with output
cargo test -- --nocapture

# Run with single thread (for integration tests)
cargo test -- --test-threads=1
```

### Writing Tests

- Add unit tests in the same file as the code
- Add integration tests in the `tests/` directory
- Use descriptive test names: `test_create_project_with_valid_name`
- Test both success and error cases
- Clean up temporary files in tests

**Test Example:**

```rust
#[test]
fn test_create_project() -> Result<()> {
    let temp_dir = TempDir::new()?;
    env::set_current_dir(temp_dir.path())?;
    
    init(None)?;
    create_project("TestProject", "medium")?;
    
    assert!(temp_dir.path().join("TestProject").exists());
    assert!(temp_dir.path().join("TestProject/README.md").exists());
    
    Ok(())
}
```

## Documentation

### Code Documentation

- Add doc comments (`///`) for public APIs
- Include examples in doc comments
- Document errors and panics
- Keep docs up-to-date with code changes

### User Documentation

- Update README.md for new features
- Add examples for new commands
- Update CHANGELOG.md
- Create guides for complex features

## Pull Request Process

1. **Update CHANGELOG.md** under "Unreleased" section
2. **Ensure all tests pass** locally
3. **Update documentation** for your changes
4. **Create the pull request** with a clear description
5. **Link related issues** using keywords (Fixes #123)
6. **Respond to review feedback** promptly
7. **Squash commits** if requested before merging

### Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing
Describe testing done

## Checklist
- [ ] Tests pass locally
- [ ] Added/updated tests
- [ ] Updated documentation
- [ ] Updated CHANGELOG.md
- [ ] Code follows style guidelines
- [ ] No new warnings from clippy
```

## Review Process

- All PRs require at least one approval
- CI must pass before merging
- Maintainers will provide feedback within 48 hours
- Be patient and respectful during review

## Release Process

Releases are managed by maintainers:

1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Create git tag: `git tag -a v0.x.0 -m "Release v0.x.0"`
4. Push tag: `git push origin v0.x.0`
5. GitHub Actions creates release automatically
6. Publish to crates.io: `cargo publish`

## Questions?

- Open a [GitHub Discussion](https://github.com/nqn/pillar/discussions)
- Comment on related issues
- Reach out to maintainers

## Recognition

Contributors will be:
- Listed in release notes
- Mentioned in the README (optional)
- Given credit in commit messages

Thank you for contributing to Pillar! 🎉
