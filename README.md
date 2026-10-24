# Ruffle snapshot prerequisite patch

This is working serializer groundwork inside upstream Ruffle, **not a completed save-state feature**. It cannot save or resume a running SWF. A regression explicitly verifies that a real Player root is rejected.

## Implemented
- Actual AVM1 ordinary-object graph capture and fresh gc-arena restoration, including cycles, shared references, property order/attributes, exact floating-point bits, and arbitrary UTF-16 units.
- Portable, versioned JSON representation containing object IDs rather than memory addresses.
- Fail-closed rejection of native payloads/functions/MovieClips, virtual properties, watchers, and interface lists. Prototype links are traversed, so standard builtin prototype chains reach native functions and are rejected too.
- Initialized AVM RNG state extraction/restoration without clock reseeding. Zero/uninitialized state cannot be captured by this API.
- A producer-process-exit/fresh-consumer-process regression for the ordinary heap graph only; this is not a movie continuation test.

## Provenance and applying
Upstream: https://github.com/ruffle-rs/ruffle
Base commit: 045db00b393aed22a462d29cb6c25efa26be90a0
Local branch: research/avm1-snapshot-graph
This repository publishes a research patch against that pinned base. No upstream pull request has been made.

Clone Ruffle separately and check out the pinned base, then apply the patch from this repository:

```powershell
git clone https://github.com/ruffle-rs/ruffle.git ruffle
cd ruffle
git checkout 045db00b393aed22a462d29cb6c25efa26be90a0
git apply --check ../ruffle-portable-states/portable-avm1-prerequisites.patch
git apply ../ruffle-portable-states/portable-avm1-prerequisites.patch
cargo test -p ruffle_core --lib portable_snapshot --no-default-features -j 1
```

Rust/Cargo 1.99.0 and Windows MSVC were used. Keep builds at one job on this 16GB host. The patch makes no desktop/frontend changes. The base's MIT/Apache-2.0 licensing applies.

## Remaining before a real execution checkpoint
MovieClip and display-list/timeline/native state; ActionScript function bodies, captured scopes and builtin bindings; interpreter globals/registers/constant pools; frame-boundary guards; timers, queues, input and virtual clock; SWF identity/assets; and backend-resource rebinding or explicit rejection policies. See SNAPSHOT_PLAN.md for the source-grounded inventory. No replay is used by this code.

The heap decoder validates structure before allocating GC objects, but deserialization input size must be bounded by its caller; it is not a hardened hostile-input container. Internal Property IDs are freshly assigned; watchers (which depend on property behavior) are unsupported. Memory-layout and cross-platform compatibility have not been tested beyond the address-free format and fresh Windows processes.

## Validation result
`cargo test -p ruffle_core --lib portable_snapshot --no-default-features -j 1` exited 0: **7 selected tests passed, 0 failed, 125 filtered out**. The selection includes a subprocess helper test; the parent process portability test additionally executes producer and consumer subprocesses. `green-test.log` is the final output; `red-build.log` records missing-API failures before implementation. The full core suite and a real SWF continuation test were not run.

Formatting, whitespace validation, and applying the complete patch against a temporary index of the pinned upstream base all passed. Patch SHA-256: `BFD87E566E2E273B5E9F3168F650D6860557E7A27E55A31CB271B3FDC5CF755F`.

Publication note: the retained build logs replace the original local checkout path with <ruffle-checkout>; test outcomes are unchanged. The patch and additions use Ruffle's MIT-or-Apache-2.0 terms; the upstream license and notices are retained in LICENSE.md.

A manual GitHub Actions workflow reproduces the pinned research checks. It has not been run on GitHub at initial publication; the recorded results above came from local runs.
