# plugin.layout — WebAssembly notice review

## Scope and evidence

The owner approved Apache-2.0 for GAMS-authored code in this repository. `LICENSE` is the same complete official Apache-2.0 text already used for the independently approved `plugin.fs`; that approval does **not** approve this repository's third-party inventory. The owner separately reviewed this repository's `NOTICE` and approved its linked-code inventory for a gated branch candidate; neither FS approval nor local success substitutes for hosted Linux review before tagging.

`src/component.c` and generated `layout_plugin.c`/component-type data are the only direct C link inputs; `src/wit/package.wit` contains no copied third-party dependency packages. The pinned toolchain uses WASI SDK 33.0, whose wasi-libc submodule is `161b3195fc2558d2b1ba3eb9ffae3b2b47407623`; the component linker is wasm-component-ld 0.5.22 and wit-bindgen 0.57.1. A local relink using the Makefile's inputs and flags plus `-Wl,-Map` produced stripped bytes identical to the pre-notice component (`6f7104b13f70d92d237e5182b52aaed187528e0599c4e2ea34a668154bc2091f`). Repeat with `scripts/record-build-evidence.sh` on hosted Linux: it compares the final notice-bearing bytes, not just the pre-notice artifact.

The relink selects WASI `crt1-reactor.o` and 37 `libc.a` members. These include `dlmalloc.c.obj`; musl-derived `memcmp.c.obj`, `strlen.c.obj`, plus stdio/string/math members; and wasi-libc-authored WASI Preview 2 glue. `NOTICE` includes wasi-libc's MIT option and license inventory, the full musl COPYRIGHT, the dlmalloc opening attribution, and conservatively the MIT text for generated binding/component support. No `libclang_rt.builtins.a` members were selected. Other build tools are not distributed.

`wasm-tools component unbundle --threshold 0` extracts one ~51 KB core and three tiny support modules (179, 101, 24 bytes). There is no `wasi_snapshot_preview1` core import or separately injected Preview 1 reactor adapter; **do not** inherit Director's adapter notice for this component. The final component imports standard WASI 0.2.6 I/O, CLI, and clock interfaces despite the GAMS-authored WIT interface having no imports. No Lua or Odin library code is linked.

## Hosted release gate

- Inspect hosted Linux `dist/evidence/link.map`, SDK VERSION, module metadata and binary equality against this reviewed inventory. If selected members or the adapter change, revise and re-review `NOTICE` and its digest before continuing.
- Review the hosted `release` branch candidate, checksums, and embedded licensing bytes before pushing a matching version tag. Local success does not authorize release.
