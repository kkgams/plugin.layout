> **Prospective v0.2.0 maintenance preparation — NOT release-ready.**
> Target: `plugin.layout` v0.2.0, `plugin.layout.wasm`, `ghcr.io/kkgams/gams/layout:0.2.0`.
> Below is frozen historical documentation: existing GitHub Release URLs,
> versioned examples and repair instructions refer to their original releases.
> For a future v0.2.0 candidate, use the prepared distribution metadata;
> rebuild and run repository-owned locked toolchain/runtime tests, review
> source drift, linked evidence and exact LICENSE/NOTICE/candidate digests.
> Migrate the direct-release publisher to draft-first immutable-policy and
> exact public-byte verification before any tag/OCI push/Pages publication.
> Do not run the historical publishing commands as v0.2.0 instructions.

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
third-party `NOTICE` and linked-code inventory were reviewed by the owner;
hosted Linux candidate evidence is still required before tagging. See `LICENSING.md`, `THIRD-PARTY-REVIEW.md`, and
`PUBLISHING.md` for the fail-closed candidate/release process.
