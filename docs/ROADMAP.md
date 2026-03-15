# RustOsLinux Delivery Roadmap

Execution is tracked in Phases 0-12 from `README.md`.
Each phase should land with:
1. A bootable or testable artifact.
2. A CI check (unit/integration/smoke).
3. A short design note under `docs/`.

## Cross-Phase Developer Environment Requirements

For Phases 6-12 to remain executable by contributors and CI, the baseline
environment must provide:

- Python installed by default.
- Rust toolchain installed by default.
- A full terminal userland with package-manager access so additional tooling
  can be installed without image rebuilds.

These requirements apply to local dev setup, CI runners, and VM/devcontainer
templates.

Implementation hooks in-repo:
- `scripts/bootstrap-dev-env.sh`
- `scripts/check-dev-env.sh`
