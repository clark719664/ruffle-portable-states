# Ruffle execution snapshot first tranche

Pinned upstream: 045db00b393aed22a462d29cb6c25efa26be90a0.

## Goal and boundary
The final goal is portable completed-frame AVM1 save, process exit, fresh process restore, and identical future behavior. This tranche implements the ordinary-object heap graph codec that goal requires. It is NOT a Player checkpoint and cannot save a running movie yet.

## Contract
Capture a root AVM1 ordinary object and all object references reachable through stored own properties. Preserve cycles, shared identity, property insertion order, attributes, exact f64 bits, arbitrary UTF-16 string code units, null and undefined. No GC addresses escape into the wire representation. Restore allocates all objects in a fresh arena before reconnecting references. Reject native objects (including functions and display objects), virtual properties, watchers, and interface lists rather than silently losing state. Validate version, references, descriptor flags and duplicate names before allocating.

## Plan
1. Add failing regression for graph capture -> JSON bytes -> fresh arena restoration.
2. Implement a versioned codec inside the actual upstream AVM1 Object implementation, not a stand-in runtime.
3. Add rejection/corruption tests and validate with one bounded core test build, jobs=1.
4. Export portable patch, exact validation output, and limitations.

## Next milestone
Add MovieClip timeline/display-list/native state and AVM1 functions/scopes with explicit backend/resource rebinding; then integrate Player frame-boundary guards, pending work/timers/RNG/input state, SWF identity, and a save/kill/restore harness. Ordinary-object support alone cannot establish deterministic continuation.

## Source-grounded continuation inventory
- `core/src/player.rs:315` Player: RNG, run state, frame accumulator/offset/rate, current-frame/phase, input and mouse state, instance IDs, timer clock, start time, compatibility settings, and frontend backends.
- `core/src/player.rs:171` GcRootData: root movie, asset library, Stage/display graph, interpreter globals/state, action queue, loaders, shared objects, variable bindings, timers, external interface, audio/video/stream managers, sockets/connections, orphans, dynamic handles, and post-frame closures.
- `core/src/avm1/runtime.rs:53` Avm1: constant pool, both case-sensitive environments, property map, stack/registers, halted flag, recursion limit, listener flag, MovieClip execution list, and bounds-compatibility flag.
- `core/src/player.rs:2061` run_frame: preload, AVM2 phases, AVM1 frame work, audio/local connections, then post-frame callbacks. A completed run_frame is a necessary boundary, but does not prove that deferred or external work is absent.
- `core/src/avm1/object/script_object.rs:108` ObjectData: native payload, ordered descriptors, interfaces, watchers. This patch handles stored descriptor graphs only and rejects the other categories.
- `core/src/avm_rng.rs:12` AvmRng contains one u32 state; state zero means lazy initialization from locale clock, which must be addressed for deterministic fresh-process continuation.

## Proposed smallest movie checkpoint policy
Start with fully preloaded offline AVM1 and a strict allowlist: no active audio/video, networking, loaders, external callbacks, pending actions, or timers; enforce this explicitly. Support root MovieClip plus all ordinary properties and AS functions/scopes reachable from it, immutable SWF assets by stable IDs/hash, timeline tag cursor/current frame/playing flags, native display state, builtin prototype bindings, AVM1 globals/registers/constant pools, RNG/input/virtual clock. A first fixture should mutate a counter in frame scripts and include a shared/cyclic object, then compare trace bytes after process kill/restart. Even this policy requires substantially more native-state work than an ordinary-object graph codec.
