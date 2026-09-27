# plugin.layout

Standalone extraction of the GAMS stateless panel-layout WebAssembly component.
It exports `gams:layout/layout@1.0.0`; layout state remains in caller-owned
layout documents.

## Build and verify

```sh
nix develop --command make test
```

The test directly invokes the transpiled component and checks initialization,
resize, and invalid input handling. Build output is `dist/plugin.layout.wasm`.

The owner approved Apache-2.0 for GAMS-authored code. The repository-specific
third-party `NOTICE` and linked-code review still require owner approval before
public source distribution. See `LICENSING.md`, `THIRD-PARTY-REVIEW.md`, and
`PUBLISHING.md` for the fail-closed candidate/release process.
