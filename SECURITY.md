# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Currently supported versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of Pillar seriously. If you believe you have found a security vulnerability, please report it to us as described below.

### Please Do Not

- **Do not** open a public GitHub issue for security vulnerabilities
- **Do not** disclose the vulnerability publicly until it has been addressed

### Please Do

1. **Email us** at [INSERT SECURITY EMAIL]
2. **Include details** about the vulnerability:
   - Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
   - Full paths of source file(s) related to the manifestation of the issue
   - The location of the affected source code (tag/branch/commit or direct URL)
   - Any special configuration required to reproduce the issue
   - Step-by-step instructions to reproduce the issue
   - Proof-of-concept or exploit code (if possible)
   - Impact of the issue, including how an attacker might exploit it

### What to Expect

- **Acknowledgment**: We will acknowledge receipt of your vulnerability report within 48 hours
- **Updates**: We will provide regular updates about our progress
- **Timeline**: We aim to resolve critical issues within 7 days
- **Credit**: We will credit you in the security advisory (unless you prefer to remain anonymous)

### Our Commitment

- We will respond to your report promptly
- We will keep you informed of our progress
- We will work with you to understand and resolve the issue quickly
- We will publicly acknowledge your responsible disclosure (if you wish)

## Security Best Practices

When using Pillar:

### File Permissions

Pillar creates files in your workspace. Ensure your workspace has appropriate permissions:

```bash
# Recommended permissions for workspace
chmod 750 .pillar/
chmod 640 .pillar/config.toml
```

### Sensitive Data

**Do not** store sensitive information (passwords, API keys, personal data) in Pillar files:

- Project descriptions
- Issue descriptions
- Comments
- Any metadata fields

Pillar files are plain text and stored unencrypted. They should be treated like any other source code file.

### Git Repositories

If using Pillar in a Git repository:

- Review your `.gitignore` to ensure sensitive files are excluded
- Be cautious when committing Pillar files to public repositories
- Consider using a separate repository for sensitive project management data

### Dependencies

We regularly audit our dependencies for security vulnerabilities:

```bash
# Check for security advisories
cargo audit
```

## Known Security Considerations

### File System Access

Pillar reads and writes files in your workspace. It:

- Only operates within the workspace directory (no directory traversal)
- Does not access files outside `.pillar/` and project directories
- Does not execute arbitrary code from files
- Does not make network requests

### Input Validation

Pillar validates all user input:

- Project/issue names are sanitized for filesystem safety
- YAML parsing is done safely using `serde_yaml`
- No eval or execution of user-provided code

### Multi-User Scenarios

Pillar is designed for single-user or small team use via Git:

- No built-in access control
- All users with filesystem access can read/modify all data
- Use OS-level permissions for access control
- Consider separate repositories for different access levels

## Security Updates

Security updates will be:

- Released as quickly as possible
- Announced in release notes
- Tagged with severity level
- Documented in CHANGELOG.md

## Version History

| Date       | Version | Issue                    | Severity |
| ---------- | ------- | ------------------------ | -------- |
| -          | -       | -                        | -        |

## Additional Resources

- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE Top 25](https://cwe.mitre.org/top25/)

## Questions?

If you have questions about this security policy, please open a GitHub Discussion or contact the maintainers.

---

Thank you for helping keep Pillar and its users safe! 🔒
