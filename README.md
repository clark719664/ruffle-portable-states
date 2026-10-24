# Actual AVM1 game checkpoint: restricted, end to end

This patch checkpoints a real running SWF in Ruffle, exits its producer process, restores into a fresh Player/GC arena, and continues with identical input-dependent behavior. It does not replay the earlier frames.

The included original collect-square fixture uses real Flash `Key.isDown` calls: left/right arrows move a cyan avatar, overlap with the gold target increments a counter, and Ruffle's AVM RNG chooses the target's next position. The movie loops every 240 frames. No external game or art assets are used.

**This is a deliberately restricted checkpoint API, not arbitrary Flash game save-state support.** A private `SealedAvm1Player` validates the movie before executing it. Unsupported tags, opcodes, calls, state and malformed snapshots return errors.

## What is preserved

- The exact immutable SWF bytes and protected numeric/Boolean root properties, using the earlier AVM1 heap codec.
- Initialized RNG state and held left/right arrow keys. Keys are delivered through real Ruffle input events.
- Root/sprite frame positions, tag-stream cursors, playback flags, fixed display structure, instance names/depths, transforms and caches, and exact AVM1 execution-list order.
- Stage background and the immutable builtin constant-pool binding. Native MovieClip objects and builtin prototypes are rebound in the fresh arena.

The ordinary-object codec still separately preserves cycles, aliases, descriptors, UTF-16 units and f64 bits. Arbitrary object graphs/functions are **not** admitted into the executing sealed movie subset.

## Supported movie contract

Uncompressed SWF 7 up to 1 MiB; solid-filled shapes; initial static placements; one-frame sprites containing shape leaves; fixed `clip_` instance names; protected `s_` numeric state; numeric arithmetic/comparisons; trace; RNG; x/y display properties; Stop/Play; and the exact readonly `Key.isDown(37/39)` native calls. Operand validation requires every action block to finish with an empty stack. Frame capture requires no queued actions, callbacks, operands, register contents, or halted VM.

Unsupported examples: ActionScript functions/scopes, general method/member calls, mutable globals/prototypes, objects/arrays in movie state, dynamic placements, nesting beyond the supported sprite/shape structure, fonts/text, bitmaps, masks/filters, sound/video, timers, network/loaders, external callbacks, wall-clock APIs, and other keyboard/mouse APIs. Frame stepping is explicit; there is no real-time tick-clock claim.

Restoration checks display identities against a static placement manifest derived from the same SWF. Snapshot fields cannot add/rename display instances to bypass the source capability policy. Transport is versioned and bounded at 8 MiB. This experimental format is pinned to the source commit below, not a stable public compatibility promise.

## Reproduce

Base: `ruffle-rs/ruffle` commit `045db00b393aed22a462d29cb6c25efa26be90a0`.

From this repository root:

```sh
git clone https://github.com/ruffle-rs/ruffle.git work/ruffle
git -C work/ruffle checkout 045db00b393aed22a462d29cb6c25efa26be90a0
git -C work/ruffle apply --check ../../portable-avm1-checkpoint.patch
git -C work/ruffle apply ../../portable-avm1-checkpoint.patch
cd work/ruffle
cargo test -p ruffle_core --lib portable_snapshot --no-default-features -j 1
```

The test module is the executable producer/consumer harness and SWF source generator. The parent test launches the producer, waits for its successful exit, and only then launches the consumer. Set `RUFFLE_PORTABLE_EXPORT` to an absolute directory to keep its artifacts. PowerShell example:

```powershell
$env:RUFFLE_PORTABLE_EXPORT = (Join-Path (Get-Location) 'checkpoint-demo')
cargo test -p ruffle_core --lib portable_snapshot --no-default-features -j 1
```

An individual phase can also be run through the compiled test executable with `RUFFLE_SEALED_DIR` set to that directory, `RUFFLE_SEALED_MODE=produce` or `consume`, and arguments:

```text
--exact player::portable_snapshot::tests::portable_snapshot_process_helper --nocapture
```

## Evidence and files

- `demo/interactive-world.swf`: generated playable fixture; use left/right arrows in Ruffle.
- `demo/checkpoint.json`: actual checkpoint after four completed frames with right arrow held.
- `demo/checkpoint.svg`, `demo/restored.svg`: before/after views from actual Ruffle draw commands, captured by a test rendering backend. These are vector-command evidence, not GPU pixel comparisons.
- `demo/expected-trace.json`, `demo/expected-final.json`, `demo/expected-render.txt`: uninterrupted producer continuation reference.
- `demo/verified.txt`: written only after the fresh consumer matches trace, complete checkpoint and actual draw-command digest.
- `fixture-source.rs`: original fixture generator excerpt; executable version is in the patch's test module.
- `green-test.log`, `provenance.json`: final executed checks and exact artifact hashes.

The suite also tests held keys, timeline looping and stopped playback, invalid formats/native state/identities/cursors, rejected instructions, and transport limits. It is a targeted suite; no full Ruffle regression-suite or arbitrary downloaded-game claim is made.

The fixture and generator are offered under the same MIT OR Apache-2.0 terms as the upstream project. `LICENSE.md` is copied from the pinned upstream. No third-party game rights are implicated.

Final recorded run: **17 selected tests passed, 0 failed, 125 filtered out**, including subprocess helpers. An independent reviewer reran the frozen binary: all 17 tests passed, exit 0, with no unresolved blocker for the sealed subset. See `independent-review.md`. Use portable-avm1-checkpoint.patch for this result. The earlier prerequisite-only publication remains available in Git history.

Publication note: retained human-readable build logs replace the original local checkout prefix with <ruffle-checkout>. Checksum-protected patches and demonstration artifacts are copied byte for byte. A manual GitHub Actions workflow fetches the pinned base, applies the current patch, requires all 17 selected tests to pass, and exports the generated checkpoint demonstration.
