## Pre-commit
```sh
pip install pre-commit
pre-commit install

pre-commit autoupdate
pre-commit run --all-files
```

## Coding Style
- rustfmt and clippy with the default settings

## Git
- rebase only, `git config merge.ff only`
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/)
- Issues and PRs can be referenced with
```
Fixes: #1337
Refs: #1234, #42
```
- A subset of https://rustc-dev-guide.rust-lang.org/contributing.html#issue-triage

## Release
- Use release-plz or cargo-release + cargo-cliff
- CHANGELOG.md follows [keep a changelog](https://keepachangelog.com/)
