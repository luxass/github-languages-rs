# use bash with strict mode
set shell := ["bash", "-eo", "pipefail", "-c"]

# Release the provided version (pass 1.2.3 or v1.2.3):
# - Normalize input to strip leading 'v' if present
# - Bump Cargo.toml using cargo set-version
# - Commit the change
# - Create an annotated tag v<version>
# Requires: cargo-edit (cargo install cargo-edit)
# Usage:
#   just release 1.2.3
#   just release v1.2.3
release version:
    @echo "Releasing version {{version}}"
    @V="{{version}}"; V="${V#v}"; \
    if command -v cargo >/dev/null 2>&1 && cargo set-version -h >/dev/null 2>&1; then \
        echo "Bumping Cargo.toml version to ${V}"; \
        cargo set-version "${V}"; \
      else \
        echo "Error: cargo set-version (from cargo-edit) is required. Install with: cargo install cargo-edit" >&2; \
        exit 1; \
      fi
    @V="{{version}}"; V="${V#v}"; \
    echo "Committing version bump"; \
    git add Cargo.toml Cargo.lock || true; \
    git commit -m "chore(release): v${V}" || echo "No changes to commit"; \
    echo "Creating tag v${V}"; \
    git tag -a "v${V}" -m "v${V}" || { echo "Tag v${V} already exists" >&2; exit 1; }
    @echo "Done. You can push with: git push && git push --tags"
