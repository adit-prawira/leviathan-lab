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
| 1 | Add baby monster in the Water | ✅ Done |
| 2 | Translate body part position | ✅ Done |
| 3 | Add color to body part | ✅ Done |
| 4 | Body Builder | 🚧 In progress |
| 5 | Add Spikes & Fins make it creepy | 🔲 |
| 6 | Add swimming animation | 🔲 |
| 7 | Polishing | 🔲 |

## Still a Hatchling for Now 🙃
<img width="1512" height="952" alt="Screenshot 2026-05-24 at 8 37 20 pm" src="https://github.com/user-attachments/assets/e238876d-9ff6-44b6-81c4-55a1d9b8db42" />

