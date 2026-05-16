# Project Leviathan Lab

Build a 3D sea monster designer with Rust + Bevy Engine.

**Goal:** Learn Rust through a fun, visual project. Every phase ships something you can see and play with.

---

## Core Idea

A creature creator but for sea monsters. Think Spore creature editor meets深海生物. Player picks, places, shapes, colors, and animates body parts to build custom sea monsters.

---

## Features (full list)

- **3D Viewport:** Orbit camera, zoom, pan. Real-time rendered scene.
- **Body Editor:** Add spheres/capsules. Select, move, scale, rotate body parts. Parts snap into a hierarchy.
- **Symmetry Mode:** Mirror edits across the left-right axis so monster stays symmetrical.
- **Color & Material:** Paint body parts with colors, roughness, metallic. Maybe simple texture slots.
- **Appendages:** Fins, tentacles, spikes, horns, teeth, eyes, claws. Placed on body surface.
- **Skeleton & Pose:** Simple joint system. Pose limbs. Idle swimming animation.
- **Save / Load:** Export monster as `.leviathan` file. Load it back. Maybe export glTF for 3D printing or rendering.
- **Randomizer:** One-button random monster generator for inspiration.
- **UI:** Mouse + keyboard controls. Minimal Bevy UI for tools, color pickers, sliders.

---

## Phase Plan (vertical slices)

Each phase delivers a complete, usable feature. No "backend first, frontend later" nonsense. Every phase ships something you can launch and interact with.

### Phase 1: Skeleton in the Water

**You can:** Launch the app. See a 3D scene with a simple sea monster shape. Orbit the camera.

- Bevy project scaffold (`cargo init`, `Cargo.toml` deps)
- Basic Bevy 3D window + PBR lighting
- A single hardcoded body (bundle of spheres/capsules)
- Orbit camera controller (mouse drag to rotate, scroll to zoom)
- Simple ocean-like background color / fog

**Ship:** A window with a 3D monster body you can look at from any angle.

---

### Phase 2: Grab a Part

**You can:** Select a body part by clicking it. Move it around with gizmos. See the monster update in real time.

- Raycast selection (click on mesh to select body part)
- Translate / rotate / scale gizmos (3D manipulators)
- Body part tree data structure (parent-child hierarchy)
- Simple Bevy UI panel showing selected part name + transform values

**Ship:** You can poke and prod the monster. Change its shape. Break it. Fix it.

---

### Phase 3: Paint the Beast

**You can:** Pick a body part and change its color. Adjust roughness and metallic. Make it shiny or slimy.

- Color picker UI (H/S/V sliders or wheel)
- Material property editing (base color, roughness, metallic)
- Symmetry painting (paint left side mirrors to right)
- Undo / redo for edit actions

**Ship:** The monster has personality now. Make it gross, shiny, dark, slimy.

---

### Phase 4: Body Builder

**You can:** Add new body parts (spheres, capsules). Remove parts. Reshape the whole creature from scratch.

- Add part tool (click to insert sphere/capsule at cursor)
- Delete selected part
- Reparent drag (drag a part onto another to make it a child)
- Resize capsule length/radius
- Part palette UI (list of available part types)

**Ship:** True creature creation. Not just editing a fixed shape — you build the whole body.

---

### Phase 5: Spikes & Fins

**You can:** Place appendages on body surface — fins, spikes, tentacles, eyes, teeth. They stick to the body part and move with it.

- Appendage placement system (click on body surface to attach)
- Appendage types: fins (flat mesh), spikes (cone), tentacles (tube/curve), eyes (sphere), teeth (small cones)
- Appendage follows parent body transform
- Surface normal alignment (appendage faces outward from body)
- Scrolling gallery UI for appendage types

**Ship:** The monster starts looking like a real creature. Add dorsal fins, tentacle beards, rows of teeth.

---

### Phase 6: Make It Swim

**You can:** Pose the monster. Make it move. Idle swimming animation. Maybe export a still frame.

- Simple skeleton / joint system for body hierarchy
- Pose mode: rotate joints (FK-style)
- Idle swimming animation: soft S-curve body wave
- Animation timeline: keyframe pose at time A and B, blend between them
- Export monster as `.leviathan` JSON file
- Load `.leviathan` file back into editor

**Ship:** The monster lives. It swims. You can save it and show your friends.

---

### Phase 7: Deep Sea Polish

**You can:** Randomize. Show off. Get surprised.

- Random monster generator (random body layout + colors + appendages)
- Water caustics / god rays / particle effects (fancy visual candy)
- UI polish: tooltips, icons, keyboard shortcuts
- Performance: LOD, culling, draw call batching
- glTF export for 3D printing / rendering in Blender
- Sound effects on UI interaction (maybe)

**Ship:** A polished app you can actually play with and share results.

---

## Tech Stack

| Layer | Choice |
|-------|--------|
| Language | Rust |
| Engine | Bevy  |
| 3D Math | `glam` (comes with Bevy) |
| UI | Bevy UI or `egui` via `bevy_egui` |
| Serialization | `serde` + `ron` or JSON |
| Gizmos | Bevy editor gizmos or `bevy_mod_gizmos` |

---

## Learning Goals

- Rust ownership, borrowing, lifetimes (gamedev forces you to learn this fast)
- ECS architecture (Bevy's way: entities, components, systems)
- 3D math: transforms, quaternions, matrices
- Real-time rendering pipeline
- Building a creative tool UX
- Save / serialization patterns in Rust

Each phase teaches a chunk. Phase 1 teaches scaffolding + 3D basics. Phase 2 teaches ECS + interaction. Phase 3 teaches asset management. And so on.
