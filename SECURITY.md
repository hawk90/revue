# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue, please report it responsibly.

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via one of the following methods:

1. **GitHub Security Advisories** (Preferred)
   - Go to [Security Advisories](https://github.com/hawk90/revue/security/advisories/new)
   - Click "Report a vulnerability"

2. **Email**
   - Send details to: security@example.com (replace with your email)

### What to Include

Please include the following information:

- Type of vulnerability (e.g., buffer overflow, SQL injection, XSS)
- Full paths of source file(s) related to the vulnerability
- Location of the affected source code (tag/branch/commit or direct URL)
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Resolution Target**: Within 90 days (depending on complexity)

### What to Expect

1. **Acknowledgment**: We will acknowledge receipt of your report
2. **Investigation**: We will investigate and validate the issue
3. **Fix Development**: We will develop a fix if confirmed
4. **Disclosure**: We will coordinate disclosure timing with you
5. **Credit**: We will credit you in the security advisory (unless you prefer anonymity)

### Safe Harbor

We consider security research conducted in accordance with this policy to be:

- Authorized concerning any applicable anti-hacking laws
- Authorized concerning any relevant anti-circumvention laws
- Exempt from restrictions in our Terms of Service that would interfere with conducting security research

We will not pursue civil action or initiate a complaint to law enforcement for accidental, good-faith violations of this policy.

## Security Best Practices for Users

When using Revue in your applications:

1. **Keep Updated**: Always use the latest version
2. **Review Dependencies**: Regularly audit your dependency tree
3. **Sanitize Input**: Always sanitize user input before display
4. **Secure Configuration**: Follow security guidelines in documentation

## Security Features

Revue includes several security features:

- No unsafe code in core library (where possible)
- Input sanitization for text widgets
- Memory-safe Rust implementation
- Regular dependency audits via `cargo-deny`

Thank you for helping keep Revue and its users safe!
