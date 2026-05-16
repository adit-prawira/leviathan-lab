# Phase 7: Deep Sea Polish

**Ship:** A polished app you can actually play with and share results.

---

## Slice 7.1: Random monster generator

- One button: "Randomize!"
- Generates random: body part count (3-8), types, sizes, positions
- Random colors + materials (hue wheel sampling)
- Random appendages placed on body
- Seeded: "Randomize again" with same seed for repro

**Accept:** Click button → new unique monster appears. Click again → different monster. No crashes.

---

## Slice 7.2: Visual effects

- Water caustics: animated light pattern projected on monster (screen-space or vertex)
- God rays / volumetrics: light shafts from above (post-process or sprite-based)
- Particle system: bubbles rising from deep, plankton floaters
- All effects toggleable (performance slider)

**Accept:** Scene looks alive. Bubbles float up. Light rays pierce down. Caustics dance on monster.

---

## Slice 7.3: UI polish

- Tooltips on all buttons and controls (hover to see description)
- Icons for tools (pencil for paint, cross for delete, plus for add)
- Keyboard shortcut labels shown next to buttons
- Consistent color scheme + font
- Loading screen or splash on startup
- Onboarding hint: "Click a part to select it"

**Accept:** UI is usable without guessing. Shortcuts visible. Tooltips explain everything.

---

## Slice 7.4: Performance

- LOD: body parts further from camera use lower-poly mesh
- Frustum culling: don't render appendages outside view
- Draw call batching: same-material parts merged
- Profiling: measure frame time, identify bottlenecks
- Graphics settings slider (low/med/high) controlling shadow quality, effects, LOD bias

**Accept:** 60fps with 20 body parts + 50 appendages. Low-end settings work on integrated GPU.

---

## Slice 7.5: glTF export

- Export current monster as `.glb` / `.gltf`
- Includes: meshes, materials (color/roughness/metallic), hierarchy
- UV mapping on procedural meshes
- Ready to import into Blender, Maya, game engines
- Export dialog: choose name + location

**Accept:** Export monster → `.glb` file. Import into Blender → looks like the in-app version.

---

## Slice 7.6: Sound effects

- UI sounds: click, tool switch, part add/delete, color pick
- Confirm dialog: save/load confirmation beeps
- Ambient: deep ocean ambience (low rumble, distant whale calls)
- Sound toggle in settings

**Accept:** Clicking in UI makes subtle sounds. Ocean ambience plays. Can mute.

---

## Learning targets

- Procedural generation algorithms
- Post-processing effects in Bevy
- Particle systems
- LOD and culling strategies
- glTF 2.0 export format
- Audio in Bevy (sprite or ECS)
- UI/UX design for creative tools

## Dependencies

| Crate | Why |
|-------|-----|
| `bevy` | Engine |
| `bevy_mod_outline` or custom | Post-process |
| `bevy_gltf` / `gltf` crate | Export |
| `bevy_kira_audio` | Sound |
| `rand` | Random generation |
