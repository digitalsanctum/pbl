# Release Process

The intent of this document is to provide a high-level overview of the release process for `pbl`.

## Overview

1. Make changes to the code.
2. Update `Cargo.toml` with the new version number.
3. Update `CHANGELOG.md` with the new version number and changes.
4. Commit changes.
5. Tag the commit with the new version number.
6. Push the commit and tag to GitHub.

Each time a tagged commit is pushed to GitHub, a GitHub action will build the code and publish the crate to [crates.io](https://crates.io).

