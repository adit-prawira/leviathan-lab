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
| Left drag (on handle) | Move / scale selected part |
| T | Translate mode |
| R | Rotate mode |
| S | Scale mode |
| Escape | Deselect |

## Tech Stack

| | |
|---|---|
| Language | Rust |
| Engine | Bevy 0.18 |
| 3D Math | `glam` (via Bevy) |
| UI | Bevy UI / `bevy_egui` (planned) |
| Serialization | `serde` + JSON (planned) |

## Status

| Phase | Name | Status |
|-------|------|--------|
| 1 | Skeleton in the Water | ✅ Done |
| 2 | Grab a Part | 🚧 In progress |
| 3 | Paint the Beast | 🔲 |
| 4 | Body Builder | 🔲 |
| 5 | Spikes & Fins | 🔲 |
| 6 | Make It Swim | 🔲 |
| 7 | Deep Sea Polish | 🔲 |

## License

MIT
