# plugin.layout context

## Language

**Layout Document**: Caller-owned value containing screen dimensions, configuration,
generation, Areas, Handles, and an optional Preview. It is passed into each update
and replaced by the returned document.

**Area**: A rectangular panel slot identified by a caller-visible, document-unique
content id.

**Handle**: A rectangular split control with boundary metadata and a caller-visible,
document-unique content id.

**Preview**: An optional validity flag and rectangle produced while evaluating a
corner operation.

**Layout Operation**: A stateless call such as initialize, resize, move, try, or
rename that returns a Layout Document plus operation metadata, or a structured
layout error.

## Relationships and boundaries

- The Plugin exports `gams:layout/layout@1.0.0` and retains no authoritative layout
  state between calls.
- Callers own Layout Documents and stable content ids; the Plugin validates document
  capacity, geometry, and identity constraints.
- Areas and Handles describe layout geometry. They do not own or render view content.
- The distribution artifact is `dist/plugin.layout.wasm`; Project installation maps
  it to `plugins/layout.comp.wasm`.
