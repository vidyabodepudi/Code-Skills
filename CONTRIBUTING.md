# Contributing to CodeHands

Thank you for your interest in contributing to CodeHands! This project is community-governed under the Apache 2.0 license, and we welcome contributions from everyone.

## How to Contribute

### Reporting Issues
- Use GitHub Issues to report bugs or suggest enhancements
- Include: what you expected, what happened, how to reproduce
- Label with: `bug`, `enhancement`, `skill`, `docs`, `security`

### Contributing Skills

1. **Read the spec** — `docs/skill-anatomy.md` defines the canonical skill format
2. **Copy the template** — `cp -r skills/_template skills/your-skill-name`
3. **Write the skill** — Follow the writing principles in `skills/writing-skills/SKILL.md`
4. **Run the tests** — `bash tests/skill-format.test.sh skills/your-skill-name`
5. **Submit a PR** — See PR requirements below

### Improving Existing Skills
- Identify a gap or improvement in an existing skill
- Create a focused PR with clear rationale
- Ensure all tests still pass after changes

### Contributing References
- References go in `references/` as standalone checklists
- Follow the existing format (checklist with checkboxes)

### Contributing Platform Adapters
- Test on the actual platform before submitting
- Document any platform-specific limitations

## Pull Request Requirements

### All PRs Must:
- [ ] Pass all three test suites:
  - `bash tests/skill-format.test.sh`
  - `bash tests/frontmatter.test.sh`
  - `bash tests/cross-reference.test.sh`
- [ ] Follow conventional commit messages: `feat(skill): add new-skill-name`
- [ ] Include a description of what changed and why
- [ ] Not introduce new dependencies (zero-dependency philosophy)

### Skill PRs Must Also:
- [ ] Include all required sections (Overview, When to Use, Process, Common Rationalizations, Red Flags, Verification, See Also)
- [ ] Have 3+ entries in the Common Rationalizations table
- [ ] Include model_variants for claude, gemini, and gpt
- [ ] Stay within the 2,000 token budget per SKILL.md
- [ ] Use `codehands:` namespace for cross-references

## Code of Conduct

We follow the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). Be respectful, constructive, and inclusive.

## Development Setup

```bash
# Clone the repository
git clone https://github.com/codehands/codehands.git
cd codehands

# Run all tests
bash tests/skill-format.test.sh
bash tests/frontmatter.test.sh
bash tests/cross-reference.test.sh

# Detect your model
bash scripts/model-detect.sh
```

## Project Governance

CodeHands is community-governed:
- **Major decisions** are made via RFC (Request for Comments) process
- **Skill ownership** — Each skill has 1-2 maintainers responsible for updates
- **Releases** follow semantic versioning
- **Security issues** should be reported privately (see SECURITY.md)

## License

By contributing, you agree that your contributions will be licensed under the Apache 2.0 License.
