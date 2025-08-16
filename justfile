# use bash with strict mode
set shell := ["bash", "-eo", "pipefail", "-c"]

# Release the provided version:
# - Bump Cargo.toml using cargo set-version
# - Commit the change
# - Create an annotated tag v<version>
# Requires: cargo-edit (cargo install cargo-edit)
# Usage:
#   just release 1.2.3
release version:
    @echo "Releasing version {{version}}"
    @if command -v cargo >/dev/null 2>&1 && cargo set-version -h >/dev/null 2>&1; then \
        echo "Bumping Cargo.toml version to {{version}}"; \
        cargo set-version "{{version}}"; \
      else \
        echo "Error: cargo set-version (from cargo-edit) is required. Install with: cargo install cargo-edit" >&2; \
        exit 1; \
      fi
    @echo "Committing version bump"
    git add Cargo.toml Cargo.lock || true
    git commit -m "chore(release): v{{version}}" || echo "No changes to commit"
    @echo "Creating tag v{{version}}"
    git tag -a "v{{version}}" -m "v{{version}}" || { echo "Tag v{{version}} already exists" >&2; exit 1; }
    @echo "Done. You can push with: git push && git push --tags"
