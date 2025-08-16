# Use bash with strict mode
set shell := ["bash", "-eo", "pipefail", "-c"]

# Bump the version in Cargo.toml to the provided version.
# Requires: cargo-edit (cargo install cargo-edit) for cargo set-version
# Usage:
#   just bump 1.2.3
bump:
    @echo "Bumping Cargo.toml version to {{version}}"
    @if command -v cargo >/dev/null 2>&1 && cargo set-version -h >/dev/null 2>&1; then \
        cargo set-version "{{version}}"; \
      else \
        echo "Error: cargo set-version (from cargo-edit) is required. Install with: cargo install cargo-edit" >&2; \
        exit 1; \
      fi

# Bump, commit, and create an annotated git tag for the provided version.
# Usage:
#   just release 1.2.3
# This will:
#   - Update Cargo.toml version to 1.2.3
#   - Commit the change
#   - Create an annotated tag v1.2.3
release:
    @echo "Releasing version {{version}}"
    just bump "{{version}}"
    @echo "Committing version bump"
    git add Cargo.toml
    git commit -m "chore(release): v{{version}}" || echo "No changes to commit"
    @echo "Creating tag v{{version}}"
    git tag -a "v{{version}}" -m "v{{version}}" || { echo "Tag v{{version}} already exists" >&2; exit 1; }
    @echo "Done. You can push with: git push && git push --tags"
