# Phase 1: Skeleton in the Water

**Ship:** A window with a 3D monster body you can look at from any angle.

---

## Slice 1.1: Project scaffold

- `cargo init --name leviathan-lab` (done)
- `Cargo.toml` with Bevy dep + version
- Rust edition 2024 or latest stable
- Verify `cargo run` opens a Bevy window (black, no content yet)
- `.gitignore` ignores `/target`

**Accept:** `cargo run` compiles and shows a Bevy window.

---

## Slice 1.2: Basic Bevy 3D window

`cargo run` shows a window with:

- Title: `Leviathan Lab`
- Default size 1280x720
- Window centered on screen
- PBR lighting: directional light + ambient light
- Camera3d at a good angle (slightly above, looking at origin)

**Accept:** Window opens with 3D lighting. Can see lit objects.

---

## Slice 1.3: Hardcoded monster body

- Skeleton data: array of body segments, each with position, rotation, scale, parent index
- Each segment is a `PbrBundle` sphere or capsule
- Segments form a simple fish-like body (head sphere, mid capsule, tail capsule)
- Child segments positioned relative to parent (transform hierarchy)

**Accept:** On launch, a recognizable fish/monster shape appears in the scene.

---

## Slice 1.4: Orbit camera controller

- Left mouse drag: orbit around target
- Scroll: zoom in/out
- Middle mouse drag: pan
- Target follows center of monster bounding box
- Smooth, no jitter

**Accept:** You can look at the monster from any angle. Zoom in close. Pan around.

---

## Slice 1.5: Ocean feel

- Background color: deep blue-black `(0.01, 0.02, 0.05)`
- Fog: exponential fog with deep blue tint
- Maybe a subtle underwater color grading

**Accept:** Scene feels underwater, not default gray.

---

## Learning targets

- Bevy App builder, systems, startup systems
- PBR lighting basics (DirectionalLight, AmbientLight)
- Transforms: translation, rotation, scale
- Parent-child hierarchy in Bevy
- Camera + projection
- Input events (mouse motion, scroll)
- Custom resources (orbit state)

## Dependencies

| Crate | Why |
|-------|-----|
| `bevy` | Engine |
