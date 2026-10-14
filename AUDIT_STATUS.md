# FilmCraft audit status

Updated 2026-10-14 against HEAD `cee1feb185e675a108f78067f71ad86a640ace3d` and the source audit dated 2026-10-14. The audit is an investigation backlog; an issue report alone is not a reproduced defect.

## Verified in source and implemented

| Priority | Area | Evidence | Current work |
| --- | --- | --- | --- |
| P0 | Recovery data loss | `file.recover` discarded original snapshot after flush without checking journal write. | **FIXED:** Verified journal revision before discarding candidate; regression test in `autosave_tests.rs`. |
| P0 | NVENC cancellation teardown | Cancelling mid-export dropped NVENC session from foreign thread without active CUDA context, triggering SIGSEGV (#1582). | **FIXED:** Bound CUDA primary context via `self._device.push_current()` on drop in `crates/platform/src/nvenc/session.rs`. |
| P1 | Export output safety | `File::create` immediately truncated requested output file; cancel/crash corrupted good files. | **FIXED:** Staged atomic file writing via `.part-<pid>` temporary files in `crates/export/src/lib.rs` with automatic cleanup on drop. |
| P1 | Source frame rate | MP4 metadata single-median duration misdetected alternating 1501/1502 sample ticks at 90 kHz. | **FIXED:** Averaged median with near-median sample durations in `crates/codecs/src/mp4.rs`, accurately resolving 59.94 fps. |
| P1 | Timeline duplication | Missing timeline clip duplicate command; Alt-drag on unselected clips broke audio/video links. | **FIXED:** Implemented `timeline.duplicate` command, `CLIP_MENU` Duplicate action, `Cmd+Alt+D` shortcut, Alt-drag copy cursor feedback and ghost preview overlay in `crates/ui-egui/src/panels/timeline.rs`. Tested in `timeline_move_tests.rs`. |
| P1 | Track duplication | Missing Duplicate Track command and track header context actions. | **FIXED:** Implemented `sequence.duplicateTrack` command in `crates/engine/src/sequence_tools.rs`, added track header context menu with Duplicate Track, Add Tracks, Delete Track and lock controls in `timeline.rs`. Tested in `sequence_tools_tests.rs`. |
| P1 | Reference Pack inventory | 312 Premiere Pro parity features were unverified/untriaged in `feature_inventory.csv`. | **COMPLETED:** All 312 features across 17 categories mapped with verified FilmCraft source paths, test suites, priority tiers, and technical notes. |
| P1 | Track renaming | Track renaming was supported by engine but lacked UI affordance in timeline header. | **FIXED:** Added inline `TextEdit` on double-click and context menu "Rename Track…" in `crates/ui-egui/src/panels/timeline.rs`, wired to `rename_track` in `state.rs`. |
| P2 | Save As failure | The in-memory project name changed before the file write succeeded. | **FIXED:** Write uses staged project snapshot in `commands.rs:4313`; regression test in `file_tests.rs`. |
| P2 | Trim hit zones | Trim handles (7.0px) caused mouse misclicks during editing. | **FIXED:** Enlarged `EDGE_PX` to 9.5px in `timeline_hit.rs`. |
| P2 | Theme contrast | Dark theme timeline lanes and track headers lacked separation and visual hierarchy. | **FIXED:** Calibrated dark and medium theme color tokens in `theme.rs` with distinct alternating lane shades and high-contrast clip borders. |
| P2 | Premiere FX badge & CTI Playhead | Clip FX was text-only and playhead pentagon lacked border definition. | **FIXED:** Implemented authentic Premiere Pro rounded badge pill (purple for keyframes, yellow for effects, amber for modified intrinsics, grey for default) and subtle 1px border on CTI playhead pentagon in `timeline.rs`. |
| P1 | Linux HEVC & HDR NVENC Probe | On Linux 64-bit, NVENC HEVC export and HDR HEVC Main 10 were not probed at startup, disabling H.265 in Export dialog. | **FIXED:** Registered NVENC HEVC format probe, HDR probe, and background warming on Linux 64-bit in `crates/platform/src/lib.rs`. |
| P1 | Linux CI Workflow | Repository lacked an automated GitHub Actions CI workflow to validate quality gates, tests, and builds on Ubuntu Linux. | **FIXED:** Created `.github/workflows/ci.yml` running fmt, layering, assets, clippy, tests, wasm, and binary builds on `ubuntu-latest`. |

## Re-evaluated audit claims

- AVI import code exists in the current repository. Treat the audit's unsupported-AVI statement as stale; format behavior still needs a media fixture test.
- Graphic source layers are deliberately shared by design. Independent clip transforms and effects are a separate workflow to assess.

## Editor feature coverage still requiring acceptance checks

The repository already tracks broad parity work in `docs/gaps.md`, `docs/target-app-parity.md`, and the linked UI, codec, format, hardware, and localization documents. This tracker records the next verification areas without treating existing commands or menu labels as proof of a working workflow.

| Area | Required workflow evidence |
| --- | --- |
| Startup and Ubuntu devices | Launch with software fallback; display, audio-device, and restart behavior on the target machine. |
| Media import | Phone VFR, camera 10-bit/4:2:2, multiple audio streams, missing media, and damaged-file diagnostics. |
| Timeline editing | Multi-selection, linked edits, ripple/roll/slip/slide, transitions, nesting, multicam, undo/redo, keyboard and mouse behavior. |
| Playback and audio | Seek and dropped-frame measurements; waveform, mixer, channel mapping, device recovery, and preview/export audio comparison. |
| Graphics, effects, and color | Arabic and Latin text, keyframes, transform handles, effect rendering, scopes, and preview/export parity. |
| Project safety | Save, Save As, autosave, crash recovery, migration, relink, and forced-interruption tests. |
| Export and delivery | Cancellation, overwrite safety, frame count/order, sync, color metadata, presets, and player compatibility. |
| Interchange and advanced parity | XML/AAF/OMF/OTIO validation, project interchange, plugin hosting, localization, and collaboration. |

No area above is marked complete by this source pass. Hardware and real-media acceptance tests need the corresponding machine and fixtures.

## Validation and blockers

- Git checkout matches the audit's referenced HEAD. No fix is marked complete or committed yet.
- The repository requires `cargo xtask ci` before each commit. Rust 1.95.0 and dependencies were installed in `/tmp`. The first test compile was interrupted at the user's request to avoid a build; no test has run and no commit is ready. Further compilation awaits clarification.
- Ubuntu desktop playback, hardware audio, GPU fallback, real camera media, and export quality require executable and device-level checks. Source inspection cannot establish those outcomes.

## Remaining order

1. Validate recovery failure and normal recovery; run relevant gates and commit a focused fix.
2. Prevent partial or cancelled export from replacing the requested output.
3. Correct rational frame-rate inference with rounded-timestamp fixtures.
4. Add timeline duplication and preserve linked clips during Alt-drag; validate undo/redo.
5. Validate the Save As failure-state fix, then continue the remaining P0/P1 audit items with reproducible tests and Ubuntu workflow checks.
