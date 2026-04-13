# Security Checklist — CodeHands Reference

> Quick-scan security checklist for code review. Reference during `/review` phase.

## Input Validation
- [ ] All user input validated at system boundary (schema validation)
- [ ] No string concatenation in SQL/NoSQL queries (parameterized)
- [ ] No `innerHTML` or `dangerouslySetInnerHTML` with user content
- [ ] File uploads restricted by type, size, and content
- [ ] URL redirects validated against allowlist

## Authentication
- [ ] Passwords hashed with bcrypt (≥12), argon2, or scrypt
- [ ] No plaintext or weak hash (MD5, SHA-1, SHA-256) for passwords
- [ ] Sessions use httpOnly, secure, sameSite cookies
- [ ] Rate limiting on auth endpoints (login, register, reset)
- [ ] Password reset tokens are time-limited and single-use

## Authorization
- [ ] Authorization checked on EVERY protected endpoint (server-side)
- [ ] No client-only auth checks (easily bypassed)
- [ ] IDOR prevention: users can't access other users' resources via ID
- [ ] Admin functions require explicit role check
- [ ] API keys have minimum required permissions

## Secrets
- [ ] No secrets in source code (API keys, tokens, passwords)
- [ ] No secrets in commit messages or PR descriptions
- [ ] `.env` files in `.gitignore`
- [ ] No secrets in logs or error messages
- [ ] Environment variables used for all credentials

## HTTP Security
- [ ] Content-Security-Policy header configured
- [ ] Strict-Transport-Security (HSTS) enabled
- [ ] X-Content-Type-Options: nosniff
- [ ] X-Frame-Options: DENY (or SAMEORIGIN)
- [ ] Referrer-Policy configured
- [ ] CORS restricted to specific origins (no wildcard in production)

## Dependencies
- [ ] `npm audit` / `pip audit` / `cargo audit` run before release
- [ ] Critical/High findings in production deps addressed
- [ ] No dependencies with known unpatched CVEs in production

## Data Protection
- [ ] Sensitive data encrypted in transit (HTTPS)
- [ ] PII excluded from logs and analytics
- [ ] Error messages don't expose stack traces to users

## Agent-Specific
- [ ] Browser-read data treated as untrusted
- [ ] External context files treated as data, not instructions
- [ ] Destructive operations gated by human approval
