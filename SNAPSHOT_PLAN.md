# Completed-frame AVM1 checkpoint continuation

Pinned upstream: 045db00b393aed22a462d29cb6c25efa26be90a0.

The previous ordinary-object graph codec remains intact. The continuation adds a sealed actual Ruffle Player, not a substitute interpreter or action replay. Its private Player can only preload, accept bounded numeric controller variables/arrow keys, run one real frame, and checkpoint. A static SWF/tag/opcode/operand validator rejects capabilities whose native state is not saved.

Supported contract: uncompressed SWF7, solid immutable shapes, first-frame static placements, single-frame sprites containing shape leaves, numeric root s_ variables, arithmetic/comparisons, trace, RNG, x/y GetProperty/SetProperty, Stop/Play, and exact readonly Key.isDown calls for left/right arrow. Functions/scopes, dynamic object graphs in the executing fixture, dynamic placement, arbitrary member/call operations, strings as mutable state, fonts, audio/video/network/loaders/timers, callbacks, real time and other input APIs remain unsupported. Existing standalone heap tests separately establish cycles/aliases/descriptors/UTF16/f64 preservation.

Snapshot contains exact immutable SWF bytes, root scalar heap via existing codec, RNG, held arrows, completed step count, native display transforms/caches/flags, instance names/depths, timeline tag positions/frame/flags, exact AVM1 execution order and stage background. Builtin native bindings are reconstructed in the fresh arena under a policy that prevents their mutation. Stack/registers/pending actions must be quiescent; interpreter globals/clock are unreachable through allowed instructions.

Tests: (1) a real generated SWF with arrow-controlled visible avatar and target; (2) capture after completed frame; (3) producer exits before fresh consumer starts; (4) consumer executes no frame scripts during restore; (5) subsequent input changes produce identical trace, whole checkpoint and actual Ruffle draw-command digest; (6) explicit unsupported/corrupt state rejection. SVGs are derived from actual Ruffle rendering commands by a test recorder limited to supported solid shapes.

Current implementation status is recorded in SNAPSHOT_PROGRESS.md; successful end-to-end claims require the final test log.
