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

No release/publish automation is included. See `LICENSING.md` and
`PREPARATION.md` before considering distribution.
