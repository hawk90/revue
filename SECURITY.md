# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 3.x     | :white_check_mark: |
| < 3.0   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue, please report it responsibly.

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via GitHub Security Advisories:

- Go to [Security Advisories](https://github.com/hawk90/revue/security/advisories/new)
- Click "Report a vulnerability"

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

## Handling a Report (Maintainers)

A vulnerability is fixed in private and disclosed after the fixed version is
out. A public issue or pull request shows the problem to everyone before users
can upgrade, so neither is used until then.

1. **Open a draft advisory.** For a report that came in through "Report a
   vulnerability", the draft already exists. For one found another way
   (review, a private message), create it yourself: Security → Advisories →
   New draft security advisory. Fill in the affected versions and the
   severity.
2. **Fix it in the advisory's private fork.** On the draft advisory, use
   "Start a temporary private fork". Write the failing test first, as for any
   bug, and review the fix there. A collaborator invited to the advisory can
   see and work on the fork.
3. **Release.** Merge the fix from the advisory and cut a patch release as
   usual. The commit and release notes say what changed in neutral terms; the
   details go in the advisory.
4. **Publish the advisory** once the release is on crates.io, crediting the
   reporter unless they asked not to be. Request a CVE from the advisory page
   if the issue is moderate or worse.
5. **Decide on RustSec.** Filing in
   [rustsec/advisory-db](https://github.com/rustsec/advisory-db) warns everyone
   running `cargo audit` or `cargo deny`. Do it when an ordinary app using
   revue is affected without doing anything unusual. It can be skipped when
   the issue needs several uncommon conditions together (one platform, an app
   passing untrusted input to a specific API, and a user action); the GitHub
   advisory then stands as the record.
6. **Record it** under [Recent Security Fixes](#recent-security-fixes) below.

If a fix has already gone through a public pull request (for a low-severity
issue, or before this process was followed), still publish the advisory after
the release, so users have a record of what was fixed and why to upgrade.

## Security Best Practices for Users

When using Revue in your applications:

1. **Keep Updated**: Always use the latest version
2. **Review Dependencies**: Regularly audit your dependency tree
3. **Sanitize Input**: Always sanitize user input before display
4. **Secure Configuration**: Follow security guidelines in documentation

## Security Features

Revue includes several security features:

- **No unsafe code** in core library (where possible)
- **Input sanitization** for text widgets
- **Memory-safe Rust** implementation
- **Regular dependency audits** via `cargo-deny`
- **Command injection protection** for accessibility backends (macOS osascript, Windows PowerShell)
- **Path traversal validation** in FilePicker to prevent directory escape attacks
- **Shell escaping utilities** (`escape_applescript()`, `escape_powershell()`) for safe command execution

### Recent Security Fixes

- **v3.7.0** (PR #896, [GHSA-369w-9xqc-8542](https://github.com/hawk90/revue/security/advisories/GHSA-369w-9xqc-8542)): Fixed command injection in `open_url` / `open_browser` on Windows
  - URLs were passed through `cmd /C start`, where characters such as `>`, `<`, `^` and `%VAR%` slipped past validation
  - Windows now launches `explorer <url>` with the URL as a single argument, without a shell
- **v2.43.1** (PR #340): Fixed command injection in accessibility backends
  - macOS osascript commands now properly escape quotes, backslashes, and control characters
  - Windows PowerShell commands use single-quote escaping for safe parameter passing
- **v2.43.0** (PR #337): Hardened FilePicker security validation
  - Added validation to prevent path traversal attacks (e.g., `../`, absolute paths)
  - Windows reserved name detection (CON, PRN, AUX, NUL, COM*, LPT*)
  - Empty path component detection

Thank you for helping keep Revue and its users safe!
