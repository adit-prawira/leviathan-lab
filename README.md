# Leviathan Lab

3D sea monster designer. Built with Rust + Bevy 0.18.

Design, sculpt, paint, and animate custom sea monsters.

## Quick Start

Requires Rust 1.80+.

```bash
cargo run
```

## Controls

### Navigation

| Input | Action |
|-------|--------|
| Right drag | Orbit camera |
| Scroll | Zoom in / out |
| Middle drag | Pan |

### Selection & Transform

| Input | Action |
|-------|--------|
| Left click | Select body part |
| T | Translate mode |
| R | Rotate mode |
| S | Scale mode |
| Escape | Deselect / back to Select mode |
| Delete / Backspace | Remove selected part |

### Add Body Part

| Input | Action |
|-------|--------|
| A | Add Part mode |
| Left click | Spawn part at cursor on Y=0 plane |
| 1-5 | Sphere, Capsule, Cone, Torus, Cylinder |

### Sculpting

| Input | Action |
|-------|--------|
| V | Sculpt mode |
| Left click + drag | Sculpt stroke on mesh surface |
| Shift + scroll | Adjust brush radius |
| 1-4 | Pull, Push, Smooth, Flatten brush |

### Symmetry

| Input | Action |
|-------|--------|
| Enable in Properties panel | Sculpt strokes mirror across X axis |

### Undo / Redo

| Input | Action |
|-------|--------|
| Cmd/Ctrl + Z | Undo |
| Cmd/Ctrl + Shift + Z | Redo |

## Tech Stack

| | |
|---|---|
| Language | Rust (Edition 2024) |
| Engine | Bevy 0.18.1 |
| 3D Math | `glam` (via Bevy) |
| UI | `bevy_egui` 0.39.1 |
| Ray Casting | `parry3d` 0.28.0 |
| Serialization | `serde` + JSON (planned) |

## Structure

```
src/
├── main.rs            — App builder, 6 custom plugins
├── lib.rs             — Module declarations
├── model/             — Pure data (BodyPart, MaterialData, Monster)
│   ├── body_part.rs   — PartType enum (5 variants) + mesh builders
│   ├── body_hierarchy.rs — parent-child tree management
│   ├── body_material.rs  — BodyMaterial Component + sync system
│   ├── material.rs    — MaterialData struct
│   └── monster.rs     — Monster + default_hatchling() 7-part fish
├── scene/             — Camera orbit + spawner
│   ├── camera.rs      — OrbitCamera resource + right-click/pan/scroll
│   └── spawner.rs     — Spawns monster to ECS
├── editor/            — Gizmos, selector, symmetry, sculpt tools
│   ├── gizmos.rs      — Translate/Rotate/Scale (T/R/S), drag handles
│   ├── resource.rs    — SystemParam bundles + shared resources
│   ├── sculpt_tool.rs — Add/delete/resize body parts
│   ├── sculpt_brush_tool.rs — Pull/Push/Smooth/Flatten vertex sculpt
│   ├── selector.rs    — Click-to-select, deselect on empty click
│   ├── symmetry.rs    — Mirror material + sculpt edits across X axis
│   └── bvh.rs         — BVH acceleration for ray-mesh intersection
├── history/           — EditHistory undo/redo (7 action types)
│   └── edit_history.rs
├── rendering/         — Lights + floor plane
│   └── screen.rs
├── ui/                — egui properties panel + toolbar
│   ├── properties.rs  — Right panel: transform, material, shape, hierarchy
│   └── toolbar.rs     — Top bar: sculpt mode / brush / part-type picker
└── plugin/            — 6 plugins wiring systems together
    ├── scene_plugin.rs
    ├── model_plugin.rs
    ├── editor_plugin.rs
    ├── history_plugin.rs
    ├── rendering_plugin.rs
    └── ui_plugin.rs
```

## Features

- **3D Viewport:** Orbit camera, zoom, pan. Real-time rendered scene.
- **Body Editor:** Add spheres/capsules/cones/toruses/cylinders. Select, move, scale, rotate body parts.
- **Hierarchy:** Parent-child body part tree with drag-and-drop reparenting.
- **Sculpting:** Pull/Push/Smooth/Flatten vertex brushes with BVH-accelerated ray casting.
- **Symmetry:** Sculpt strokes and material edits mirror across X axis.
- **Color & Material:** Paint body parts with colors, roughness, metallic.
- **Undo / Redo:** Full undo/redo for all edit actions including sculpt strokes.
- **Resize Guard:** Resize sliders disabled on sculpted parts — "Reset Sculpt to Resize" with undo support.

## Status

| Phase | Name | Status |
|-------|------|--------|
| 1 | Skeleton in the Water | Done |
| 2 | Grab a Part | Done |
| 3 | Paint the Beast | Done |
| 4 | Body Builder | Done |
| 5a | Sculpt the Flesh (base shapes, BVH, brushes, undo) | Done |
| 5b | Sculpt the Flesh (symmetry, precision, resize guard) | Done |
| 6 | Make It Swim | Planned |
| 7 | Deep Sea Polish | Planned |

## Still a Hatchling for Now?🙃
<img width="1512" height="950" alt="Screenshot 2026-06-09 at 11 32 48 pm" src="https://github.com/user-attachments/assets/ef49abff-7ea3-42ec-8621-0c8537e4a664" />

