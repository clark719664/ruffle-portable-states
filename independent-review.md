# Independent Ruffle endpoint review — approved for the sealed subset

Final frozen executable independently rerun on 2026-10-23:
`target/debug/deps/ruffle_core-5d89497700d6581e.exe portable_snapshot --nocapture --test-threads=1`
Result: 17 passed, 0 failed, 0 ignored, 125 filtered; process exit 0.
Executable SHA256: EA46CF05B6E00950A4ACB239A2C839108C4B57F62732A6B169235EF0915C0F13.

No unresolved blocking finding in the reviewed scope. Initial findings were fixed and independently source-reviewed: static placement manifest now checks child IDs/depths/names/topology/place-frame before restoration; checkpoint enforces the same 8MiB transport limit as restore; sealed variable-name validation matches the bounded namespace. Root name is checked against the pristine root.

Evidence:
- Actual Ruffle Player is private. Static SWF7 policy accepts bounded immutable shape/sprite placement and a closed operand-checked opcode subset. Branches, constant-pool changes, getters/setters, function/object mutation, dynamic targets, timers, networking and unsupported native state are rejected. Native Key.isDown is admitted only for literal arrow keys; native aliases cannot enter the scalar heap.
- Restore creates a fresh arena, preloads immutable resources, installs serialized heap/display/timeline/RNG/input, checks quiescence and returns only on success. There is no run_frame replay. Fixture traces make any replay observable.
- Independent tests cover producer exit before fresh consumer, real arrow input, actual avatar/goal transforms, another collection and RNG respawn after restoration, complete checkpoint/trace equality and actual native draw-command equality. The renderer sees two real shapes.
- Stopped frames, crossing frame 240, capturing/restoring again after wrap, held keys, exact transform bits and malformed names/IDs/depths/cursors/flags/RNG/pool/version/oversize are exercised. LOOP_QUEUED is serialized and accepted only for multi-frame clips; frame cursor validation uses real ShowFrame and terminal zero-length End boundaries.
- Exported patch contains all endpoint/policy/render-probe/test modules, display codec, heap adapters, RNG and integration changes. Demo directory contains SWF, checkpoint, expected continuation/renderer evidence and before/after SVG.

Approval is limited to this explicitly sealed AVM1 subset. This does not establish arbitrary SWF/game save-state coverage or third-party game compatibility.
