# Leviathan Lab

3D sea monster designer. Built with Rust + Bevy 0.18.

Design, sculpt, paint, and animate custom sea monsters.

## Quick Start

Requires Rust 1.80+.

```bash
cargo run
```

## Controls

| Input | Action |
|-------|--------|
| Right drag | Orbit camera |
| Scroll | Zoom in / out |
| Middle drag | Pan |
| Left click | Select body part |
| Left drag (on handle) | Move selected part |
| T | Translate mode |
| R | Rotate mode |
| S | Scale mode |
| Escape | Deselect / back to Select mode |
| Delete / Backspace | Remove selected part (placeholder) |
| A | Add Part mode |
| Left click (Add Part mode) | Spawn part at cursor on Y=0 plane |
| Cmd/Ctrl + Z | Undo material change |
| Cmd/Ctrl + Shift + Z | Redo material change |

## Tech Stack

| | |
|---|---|
| Language | Rust |
| Engine | Bevy 0.18 |
| 3D Math | `glam` (via Bevy) |
| UI | `bevy_egui` 0.39 |
| Serialization | `serde` + JSON (planned) |

## Structure

```
src/
├── model/       — BodyPart, BodyMaterial, Monster data
├── editor/      — gizmos, selector, symmetry, add_part
├── history/     — undo/redo (EditHistory)
├── rendering/   — screen lights
├── scene/       — camera, spawner
└── ui/          — properties panel
```

## Status

| Phase | Name | Status |
|-------|------|--------|
| 1 | Skeleton in the Water | ✅ Done |
| 2 | Grab a Part | ✅ Done |
| 3 | Paint the Beast | ✅ Done |
| 4 | Body Builder | 🚧 In progress |
| 5 | Spikes & Fins | 🔲 |
| 6 | Make It Swim | 🔲 |
| 7 | Deep Sea Polish | 🔲 |


