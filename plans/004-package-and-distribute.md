# Plan: Package and Distribute Pillar

**Status**: Draft  
**Created**: 2025-12-29  
**Updated**: 2025-12-29

## Overview

Prepare Pillar for open source distribution with comprehensive documentation, automated releases, and multi-platform distribution channels. Make it easy for users to discover, install, and contribute to the project.

## Goals

1. Set up professional GitHub repository with complete documentation
2. Choose appropriate open source license
3. Implement automated CI/CD pipeline for testing and releases
4. Distribute via multiple channels (crates.io, Homebrew, binaries)
5. Create contributor-friendly development environment
6. Establish version management and changelog practices

## Non-Goals

- GUI or web interface (CLI only)
- Paid/commercial licensing (fully open source)
- Package managers beyond Homebrew/Cargo initially (apt/yum can come later)
- Windows installer (provide binary downloads initially)

## Distribution Channels

### 1. Crates.io (Primary)
```bash
cargo install pillar-cli
```
- Official Rust package registry
- Automatic version updates
- Source distribution

### 2. Homebrew (macOS/Linux)
```bash
brew install pillar
```
- Tap: `homebrew-pillar` or submit to homebrew-core
- Binary distribution
- Automatic updates via `brew upgrade`

### 3. GitHub Releases
- Pre-built binaries for:
  - macOS (Intel + Apple Silicon)
  - Linux (x86_64, ARM64)
  - Windows (x86_64)
- Checksums for verification
- Release notes with each version

### 4. Docker (Optional)
```bash
docker run -v $(pwd):/workspace pillar/pillar init
```
- Containerized environment
- Cross-platform consistency

## Repository Structure

```
pillar/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml              # Test on every PR/push
│   │   ├── release.yml         # Build binaries on tag
│   │   └── audit.yml           # Security audit
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── pull_request_template.md
├── src/                        # Source code
├── tests/                      # Integration tests
├── docs/
│   ├── getting-started.md
│   ├── commands.md
│   ├── configuration.md
│   └── architecture.md
├── examples/                   # Example workflows
├── scripts/
│   ├── install.sh              # Cross-platform installer
│   └── release.sh              # Release automation
├── .gitignore
├── Cargo.toml
├── Cargo.lock
├── LICENSE                     # MIT or Apache-2.0
├── README.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
└── SECURITY.md
```

## Documentation

### README.md Structure

1. **Banner/Logo** - Project name and tagline
2. **Badges** - Build status, version, license, downloads
3. **Quick Start** - Installation and basic usage
4. **Features** - Key capabilities with examples
5. **Installation** - All distribution methods
6. **Usage** - Common commands and workflows
7. **Documentation** - Links to detailed docs
8. **Contributing** - How to get involved
9. **License** - License information
10. **Acknowledgments** - Credits and inspiration

### Additional Documentation

**CONTRIBUTING.md**
- Development setup instructions
- Code style guidelines
- Testing requirements
- Pull request process
- Issue reporting guidelines

**CHANGELOG.md**
- Keep a Changelog format
- Semantic versioning
- Migration guides for breaking changes

**CODE_OF_CONDUCT.md**
- Use Contributor Covenant
- Clear expectations for community behavior

**SECURITY.md**
- Security policy
- How to report vulnerabilities
- Supported versions

## License Selection

### Recommended: MIT License

**Pros:**
- Simple and permissive
- Well-understood by community
- Compatible with most projects
- No patent clause complications
- Industry standard for CLI tools

**Alternative: Apache 2.0**
- Explicit patent grant
- More comprehensive
- Better for larger organizations

**Decision:** MIT License for simplicity and adoption

## Version Management

### Semantic Versioning (SemVer)

Format: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking changes (e.g., CLI argument changes)
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

### Version 1.0.0 Criteria

Before declaring 1.0.0, ensure:
- [ ] All core features implemented and tested
- [ ] API/CLI interface is stable
- [ ] Documentation is comprehensive
- [ ] At least 3 months of beta testing
- [ ] No critical bugs
- [ ] Migration path from 0.x defined

### Pre-1.0 Versioning

Start at `0.1.0`:
- `0.1.0` - Initial release (current state)
- `0.2.0` - Comments feature added
- `0.3.0` - Next major feature
- `1.0.0` - Stable API commitment

## CI/CD Pipeline

### GitHub Actions Workflows

#### 1. Continuous Integration (ci.yml)

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
      - run: cargo test --all-features
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install cargo-tarpaulin
      - run: cargo tarpaulin --out Xml
      - uses: codecov/codecov-action@v3
```

#### 2. Release Automation (release.yml)

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: x86_64-apple-darwin
            os: macos-latest
          - target: aarch64-apple-darwin
            os: macos-latest
          - target: x86_64-pc-windows-msvc
            os: windows-latest
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - run: cargo build --release --target ${{ matrix.target }}
      - name: Create archive
        run: |
          cd target/${{ matrix.target }}/release
          tar czf pillar-${{ matrix.target }}.tar.gz pillar
      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: pillar-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/pillar-${{ matrix.target }}.tar.gz

  release:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v3
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            **/pillar-*.tar.gz
          generate_release_notes: true
```

#### 3. Security Audit (audit.yml)

```yaml
name: Security Audit

on:
  schedule:
    - cron: '0 0 * * 0'  # Weekly
  pull_request:

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: rustsec/audit-check@v1
```

## Cargo.toml Metadata

Update for crates.io publishing:

```toml
[package]
name = "pillar-cli"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <email@example.com>"]
description = "A file-based task tracker for managing projects, milestones, and issues"
documentation = "https://github.com/yourusername/pillar"
homepage = "https://github.com/yourusername/pillar"
repository = "https://github.com/yourusername/pillar"
readme = "README.md"
license = "MIT"
keywords = ["task", "project-management", "cli", "productivity", "markdown"]
categories = ["command-line-utilities", "development-tools"]
exclude = [
    "tests/fixtures/*",
    ".github/*",
    "docs/*",
]

[badges]
maintenance = { status = "actively-developed" }

[[bin]]
name = "pillar"
path = "src/main.rs"
```

## Homebrew Formula

Create `homebrew-pillar` tap or submit to homebrew-core:

```ruby
class Pillar < Formula
  desc "File-based task tracker for projects, milestones, and issues"
  homepage "https://github.com/yourusername/pillar"
  url "https://github.com/yourusername/pillar/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "..."
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/pillar", "--version"
    system "#{bin}/pillar", "init"
    assert_predicate testpath/".pillar", :exist?
  end
end
```

## Release Process

### 1. Pre-Release Checklist

- [ ] All tests passing
- [ ] Update CHANGELOG.md
- [ ] Update version in Cargo.toml
- [ ] Update version in README examples
- [ ] Run `cargo update` to update Cargo.lock
- [ ] Test installation from source
- [ ] Review documentation for accuracy

### 2. Create Release

```bash
# Update version
vim Cargo.toml  # Update version
vim CHANGELOG.md  # Document changes

# Commit changes
git add .
git commit -m "chore: release v0.1.0"

# Create tag
git tag -a v0.1.0 -m "Release v0.1.0"

# Push
git push origin main --tags
```

### 3. Post-Release

- [ ] Verify GitHub release created
- [ ] Test binary downloads
- [ ] Publish to crates.io: `cargo publish`
- [ ] Update Homebrew formula
- [ ] Announce on social media/forums
- [ ] Update project website (if applicable)

## Crates.io Publishing

### Initial Setup

```bash
# Login to crates.io
cargo login <your-api-token>

# Dry run
cargo publish --dry-run

# Publish
cargo publish
```

### Publishing Checklist

- [ ] Run `cargo package` to verify what gets published
- [ ] Ensure README.md is included
- [ ] Verify all examples work
- [ ] Check crates.io page after publishing
- [ ] Test installation: `cargo install pillar-cli`

## Community Building

### Communication Channels

1. **GitHub Discussions** - Q&A, ideas, announcements
2. **Discord/Slack** (optional) - Real-time community chat
3. **Twitter/Mastodon** - Updates and tips
4. **Blog** (optional) - Deep dives and tutorials

### Promotion Strategy

1. **Launch Announcement**
   - Post on Reddit (r/rust, r/commandline)
   - Hacker News
   - Dev.to article
   - Rust newsletter submission

2. **Content Creation**
   - Video demo/tutorial
   - Blog post: "Building a Task Tracker in Rust"
   - Comparison with similar tools

3. **Community Engagement**
   - Respond to issues promptly
   - Welcome first-time contributors
   - Recognize contributors in releases
   - Share user stories/use cases

## Metrics and Analytics

### Track

- **Downloads**: crates.io stats, GitHub release downloads
- **Stars/Forks**: GitHub metrics
- **Issues/PRs**: Community engagement
- **Installation success rate**: Via install script telemetry (opt-in)

### Goals (First 6 Months)

- 100+ GitHub stars
- 1,000+ downloads
- 10+ contributors
- 50+ issues/PRs
- Listed in "awesome" lists

## Legal Considerations

### Dependencies

- [ ] Review all dependency licenses
- [ ] Ensure compatibility with chosen license
- [ ] Document dependency licenses in NOTICE file (if required)
- [ ] Run `cargo license` to generate license list

### Trademark

- [ ] Check name availability
- [ ] Consider trademark registration (optional)
- [ ] Add trademark notice to README (if applicable)

## Implementation Steps

1. **Repository Setup** (1 day)
   - Create GitHub repository
   - Add .gitignore, LICENSE, README
   - Set up branch protection rules

2. **Documentation** (2 days)
   - Write comprehensive README
   - Create CONTRIBUTING.md
   - Add CODE_OF_CONDUCT.md
   - Write getting-started guide

3. **CI/CD Setup** (1 day)
   - Configure GitHub Actions
   - Set up automated testing
   - Configure release automation

4. **Package Metadata** (1 hour)
   - Update Cargo.toml with metadata
   - Add keywords and categories
   - Verify package contents

5. **Pre-Release Testing** (1 day)
   - Test on multiple platforms
   - Verify installation methods
   - Fix any blocking issues

6. **Initial Release** (1 hour)
   - Tag v0.1.0
   - Publish to crates.io
   - Create GitHub release

7. **Homebrew Distribution** (2 days)
   - Create formula
   - Test installation
   - Submit to tap or homebrew-core

8. **Launch Promotion** (1 day)
   - Write announcement posts
   - Submit to communities
   - Share on social media

## Maintenance Plan

### Regular Tasks

**Weekly:**
- Review and respond to issues/PRs
- Monitor CI failures
- Update dependencies

**Monthly:**
- Review and merge dependency updates
- Analyze usage metrics
- Plan next release features

**Quarterly:**
- Security audit
- Dependency cleanup
- Documentation review

### Long-term Roadmap

**Q1 2026:**
- Release v0.2.0 with comments
- Reach 100 stars
- Establish contributor community

**Q2 2026:**
- Release v0.3.0 with new features
- Submit to homebrew-core
- Create video tutorials

**Q3 2026:**
- Release v0.9.0 (release candidate)
- Beta testing period
- Performance optimization

**Q4 2026:**
- Release v1.0.0
- Stability guarantee
- Long-term support commitment

## Success Criteria

- [ ] Published to crates.io
- [ ] Available via Homebrew
- [ ] Automated CI/CD pipeline working
- [ ] 90%+ test coverage
- [ ] Comprehensive documentation
- [ ] 3+ external contributors
- [ ] 50+ GitHub stars
- [ ] Active issue management (response within 48h)
- [ ] Regular releases (monthly during 0.x)

## Open Questions

1. Should we provide apt/yum packages from day one? (Decision: No, start simple)
2. Docker image necessary initially? (Decision: Optional, can add later)
3. Submit to homebrew-core vs own tap? (Decision: Own tap first, homebrew-core after proven)
4. Pricing model if commercial version? (Decision: Fully open source, no commercial version)

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Name conflict | High | Research thoroughly before launch |
| Low adoption | Medium | Invest in marketing and community |
| Security vulnerabilities | High | Regular audits, fast patch releases |
| Maintainer burnout | High | Set boundaries, recruit co-maintainers |
| Breaking changes anger users | Medium | Clear communication, migration guides |

## Changelog

- 2025-12-29: Initial plan created
