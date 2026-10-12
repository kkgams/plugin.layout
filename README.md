## Current 0.2.1 release authorization

The owner's current instruction is: "no need for review of checksums / licenses - those area already prepared - drop pending reviews, commit changes, prepare tags - create script (using gh) to push to repos and create releases."

This supersedes earlier pending human checksum/license and hosted-review approvals
for the prepared 0.2.1 release. Documentation commits and annotated **local**
`v0.2.1` tags at the tested HEAD are authorized. No repeated human review is
pending for these unchanged prepared bytes. Prior "before any tag" restrictions,
including publisher migration, do not restrict local tagging.

This is not evidence of hosted Linux CI, GUI/Host consumption, registry visibility,
immutable policy, draft-first rehearsal or anonymous public-byte verification.
Those coverage gaps remain accurately unverified; publication technical checks
still apply. Version, source closure, evidence, exact license/notice digest and
candidate-byte gates remain strict. No source or legal evidence is refreshed.
No remote push/publication, repository variable or remote configuration change
is performed by this local preparation. Never move or overwrite an existing tag.

> Prepared target: `plugin.layout` v0.2.1; not yet published.
> The documentation below is a frozen historical record, not current
> pending approval requirements. Its human review/local-tag restrictions are
> superseded above; technical publication safeguards are not waived.

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
