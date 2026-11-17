---
title: Stateless Layout
description: Caller-owned rectangular panel layout documents.
status: reference
---
# Stateless Layout

The stateless layout component is a [GAMS](https://github.com/kkgams/gams) WASM Project Unit exporting `gams:layout/layout@1.0.0`. Callers own layout documents, Areas, Handles and stable content IDs; each operation receives a document and returns the next document plus operation metadata or a structured error. The component calculates geometry, not View content.

| Unit | Source / output | Original work |
| --- | --- | --- |
| Stateless panel layout | `src/component.c`, `src/wit/package.wit`; `build.nosync/plugin.layout.wasm` | GAMS contributors; WASI SDK/runtime contributors |

## Development

Install Nix; run native commands with `nix develop --command make <target>`. The flake pins WASI SDK, binding tools and Rust; Cargo locks the native Wasmtime runtime. Dependency-free Node runs only the pinned wiki engine.

| Command | Purpose |
| --- | --- |
| `make build` | Compile the layout component under `build.nosync/`. |
| `make package` | Build raw WASM, LICENSE, NOTICE and SHA256SUMS; unchanged output does no work. |
| `make test` | Run native Cargo/Wasmtime initialization, resize and invalid-dimension assertions against a separate test component. |
| `make open` | Build and open local `gams.json` in the released macOS Apple Silicon Host. |
| `make serve-docs` | Serve pinned wiki; independent `DOCS_PORT` defaults to 8080. |
| `make docs` | Build documentation into `build.nosync/site/`. |
| `make validate-docs` | Validate documentation. |
| `make clean` | Remove all of `build.nosync/`, `tools/` and `gams_modules/`, including unmarked contents; refuse tracked files and symlinks before deleting anything. |

The local Project uses the built layout component, its pinned layout UI Service and the published Files View; it is not a geometry/API demo. The Host downloads only when absent, retains its notices and is not updated implicitly. Native tests do not cover Host GUI interactions or every layout operation. Supporting pins and caller-owned state remain unchanged.
