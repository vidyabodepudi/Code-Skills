---
name: security-and-hardening
description: >
  OWASP-aligned security engineering for all code produced by agents.
  Use when building any feature that handles user input, authentication,
  data storage, API endpoints, file uploads, or external integrations.
  Covers OWASP Top 10, injection prevention, secrets management, CSP,
  dependency auditing, and security review checklists.
version: 1.0.0
permissions:
  - fs.read
  - fs.write
  - shell.exec
  - net.http
triggers:
  - context: "handling user input"
  - context: "building authentication"
  - context: "building API endpoints"
  - context: "storing sensitive data"
  - context: "file uploads"
  - context: "external API integrations"
  - command: "/review"
platforms:
  - claude-code
  - cursor
  - gemini-cli
  - copilot
  - codex
  - opencode
  - windsurf
  - kiro
dependencies:
  - test-driven-development@^1.0.0
model_variants:
  claude:
    enforcement_style: "human-partner collaborative"
    security_phrasing: "Security is a shared responsibility. Let's review this together before shipping."
  gemini:
    enforcement_style: "structured-checklist"
    security_phrasing: "SECURITY GATE: ☐ Input validation ☐ Output encoding ☐ Auth checks ☐ Secrets audit ☐ Headers configured"
  gpt:
    enforcement_style: "directive-imperative"
    security_phrasing: "You MUST validate all inputs, encode all outputs, and verify all auth before declaring this secure."
author: codehands-core
signed: true
tier: 1
---

## Overview

Security is not a feature — it's a constraint that applies to every feature. This skill ensures all agent-produced code follows security best practices aligned with the OWASP Top 10 (2025). Every input is untrusted. Every output is encoded. Every secret is protected.

## When to Use

- When building ANY feature that handles user input
- When implementing authentication or authorization
- When building API endpoints
- When storing or transmitting sensitive data
- When integrating with external services or APIs
- When reviewing existing code for security issues
- During `/review` phase before shipping
- NOT for pure computation or display-only code with no external input

## Process

### Phase 1: Threat Assessment

1. **Identify trust boundaries.** Where does untrusted data enter the system?
   - User input (forms, URL parameters, headers, cookies)
   - External API responses
   - File uploads
   - Database query results (yes — they can be tampered with)
   - Environment variables from untrusted sources

2. **Map data flow.** Trace untrusted data from entry to storage/display:
   ```
   User Input → Validation → Processing → Output (DB / API / HTML)
   ```
   Security controls MUST exist at every arrow (→) in this chain.

### Phase 2: Input Defense

3. **Validate all inputs at the system boundary.** Use schema validation libraries:
   ```typescript
   // GOOD: Schema validation at the boundary
   import { z } from 'zod';
   const UserInput = z.object({
     email: z.string().email().max(254),
     name: z.string().min(1).max(100).regex(/^[\w\s\-']+$/),
     age: z.number().int().min(0).max(150)
   });
   const validated = UserInput.parse(req.body); // throws on invalid

   // BAD: Manual validation scattered through logic
   if (req.body.email && req.body.email.includes('@')) { ... }
   ```

4. **Prevent injection attacks.** SQL, NoSQL, OS command, LDAP:
   ```typescript
   // GOOD: Parameterized query
   const user = await db.query('SELECT * FROM users WHERE id = $1', [userId]);

   // BAD: String concatenation (SQL injection)
   const user = await db.query(`SELECT * FROM users WHERE id = ${userId}`);
   ```
   String concatenation for SQL queries is a **RED FLAG**. Always use parameterized queries or an ORM.

5. **Prevent XSS.** Encode all output rendered in HTML:
   ```typescript
   // GOOD: Framework auto-escaping (React, Angular, Vue)
   return <div>{userComment}</div>;  // React auto-escapes

   // BAD: Dangerous HTML insertion
   element.innerHTML = userComment;  // XSS vulnerability
   ```

### Phase 3: Authentication & Authorization

6. **Password hashing.** bcrypt with cost factor ≥ 12. Never MD5, SHA-1, or SHA-256 for passwords:
   ```typescript
   import bcrypt from 'bcrypt';
   const hash = await bcrypt.hash(password, 12);
   const valid = await bcrypt.compare(input, hash);
   ```

7. **Session security.** Cookies must be:
   - `httpOnly: true` — prevents JavaScript access
   - `secure: true` — HTTPS only
   - `sameSite: 'strict'` or `'lax'` — prevents CSRF
   - Short-lived with refresh tokens for long sessions

8. **Authorization on every endpoint.** Check permissions server-side, not just client-side:
   ```typescript
   // GOOD: Server-side authorization check
   app.get('/admin/users', requireRole('admin'), listUsers);

   // BAD: Client-side only check (easily bypassed)
   if (user.role === 'admin') { showAdminPanel(); }
   ```

9. **Rate limiting.** Apply to authentication endpoints, API endpoints, and any resource-intensive operations.

### Phase 4: Secrets Management

10. **Secrets in environment variables, NEVER in code:**
    ```bash
    # GOOD: Environment variable
    DATABASE_URL=postgres://user:pass@host/db

    # BAD: Hardcoded in source (will be committed)
    const DB_URL = "postgres://user:pass@host/db";
    ```

11. **Audit for leaked secrets.** Before every commit, check:
    - No API keys, tokens, or passwords in code
    - No secrets in commit messages
    - `.env` files are in `.gitignore`
    - No secrets in logs or error messages

### Phase 5: HTTP Security Headers

12. **Configure security headers:**
    ```
    Content-Security-Policy: default-src 'self'; script-src 'self'
    Strict-Transport-Security: max-age=31536000; includeSubDomains
    X-Content-Type-Options: nosniff
    X-Frame-Options: DENY
    Referrer-Policy: strict-origin-when-cross-origin
    Permissions-Policy: camera=(), microphone=(), geolocation=()
    ```

13. **CORS restriction.** Never use `Access-Control-Allow-Origin: *` in production. Whitelist specific origins.

### Phase 6: Dependency Auditing

14. **Audit dependencies before shipping:**
    ```bash
    npm audit           # Node.js
    pip audit           # Python
    cargo audit         # Rust
    ```

15. **Triage audit findings:**
    - **Critical/High in production dependency → Block release.** Fix or replace.
    - **Critical/High in dev dependency → Fix in current sprint.**
    - **Moderate/Low → Schedule fix.** Don't ignore, but don't block.
    - **No fix available → Document the risk** and add compensating controls.

### Phase 7: File Uploads (if applicable)

16. **Restrict by type, size, and content:**
    - Validate MIME type AND file extension (both can be spoofed — check both)
    - Enforce maximum file size
    - Scan content (never trust the extension alone)
    - Store uploaded files OUTSIDE the web root
    - Generate random filenames — never use the user's filename

## Common Rationalizations

| Rationalization | Reality |
|---|---|
| "This is an internal app, security doesn't matter" | Internal apps get compromised. Internal users can be malicious. Internal networks get breached. Secure everything. |
| "We can add security later" | Security retrofits cost 10-100x more than building it in. Insecure code accrues vulnerabilities that compound. Secure now. |
| "The framework handles security for me" | Frameworks provide tools, not guarantees. You must use them correctly. Auto-escaping only works if you don't bypass it with `dangerouslySetInnerHTML` or `innerHTML`. |
| "This input is validated on the client" | Client-side validation is UX, not security. Anyone can bypass it with a browser console or curl. Server-side validation is mandatory. |
| "npm audit has too many findings to fix" | Triage by severity and dependency type. Critical/High in production deps = block release. Low/Moderate in dev deps = schedule. Ignoring all findings because there are many is not a strategy. |
| "nobody would actually exploit this" | "Nobody would" is the most expensive assumption in security. Automated scanners find and exploit vulnerabilities in minutes. |
| "The data isn't sensitive" | All user data has privacy implications. Metadata (timestamps, IP addresses, access patterns) can be sensitive. When in doubt, protect it. |

## Red Flags

- User input passed directly to SQL queries, shell commands, or HTML rendering
- Hardcoded secrets, API keys, or credentials in source code
- Passwords stored in plaintext or with weak hashing (MD5, SHA-1)
- Missing server-side authorization checks (client-only auth)
- CORS set to `*` in production
- `innerHTML` or `dangerouslySetInnerHTML` with user content
- Missing security headers
- `.env` files not in `.gitignore`
- `npm audit` / `pip audit` never run
- File uploads stored in the web root with user-provided filenames

## Verification

- [ ] All user inputs validated at the system boundary with schema validation
- [ ] No SQL/NoSQL/command injection vectors (parameterized queries used)
- [ ] No XSS vectors (output encoding on all user-controlled content)
- [ ] Passwords hashed with bcrypt (cost ≥ 12), argon2, or scrypt
- [ ] Sessions use httpOnly, secure, sameSite cookies
- [ ] Authorization checked on every protected endpoint (server-side)
- [ ] No secrets in source code, logs, or error messages
- [ ] Security headers configured (CSP, HSTS, X-Frame, etc.)
- [ ] CORS restricted to specific origins
- [ ] Dependencies audited and critical/high findings addressed
- [ ] Rate limiting on authentication and resource-intensive endpoints

## See Also

- `codehands:agent-security` — Security of the agent itself (OWASP LLM Top 10)
- `codehands:agent-security` — Security of the skill supply chain
- `references/security-checklist.md` — Quick-reference security checklist
- `references/owasp-llm-top10.md` — LLM-specific security risks
